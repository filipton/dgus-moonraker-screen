use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use chrono::{DateTime, Local, Timelike};
use moonraker_api::MoonrakerMsg;
use rppal::uart::Uart;
use serial_utils::{construct_change_page, construct_get_page, construct_i16, construct_text};
use structs::{PrinterStateResult, ScreenState};
use tokio::{
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        Mutex, RwLock,
    },
    time::Instant,
};

mod serial_utils;
mod structs;

const RETRY_TIMEOUT: u64 = 5000;
const BOOT_TIMEOUT: u128 = 1000;
const TIMEOUT_THRESHOLD: u128 = 5000;

// TODO: make this configurable (or just hardcode it as localhost)
pub const MOONRAKER_API_URL: &str = "192.168.1.18:7125";

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = moonraker_api::connect(MOONRAKER_API_URL).await?;
    let rx = Arc::new(Mutex::new(rx));

    let mut objects: HashMap<String, Option<Vec<String>>> = HashMap::new();
    objects.insert("display_status".to_string(), None);
    objects.insert("print_stats".to_string(), None);
    objects.insert(
        "extruder".to_string(),
        Some(vec!["target".into(), "temperature".into()]),
    );
    objects.insert(
        "heater_bed".to_string(),
        Some(vec!["target".into(), "temperature".into()]),
    );

    // subscribe to printer updates
    tx.send(MoonrakerMsg::new_param_id(
        moonraker_api::methods::MoonrakerMethod::PrinterObjectsSubscribe,
        moonraker_api::params::MoonrakerParam::PrinterObjectsSubscribe { objects },
    ))?;

    loop {
        let res = connect_to_serial(&tx, rx.clone()).await;
        if let Err(_) = res {
            // retry after 5 seconds
            tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_TIMEOUT)).await;
        }
    }
}

async fn connect_to_serial(
    moonraker_tx: &UnboundedSender<MoonrakerMsg>,
    moonraker_rx: Arc<Mutex<UnboundedReceiver<MoonrakerMsg>>>,
) -> Result<()> {
    let serial = rppal::uart::Uart::new(115200, rppal::uart::Parity::None, 8, 1);
    if let Err(e) = serial {
        return Err(anyhow::anyhow!("Error: {}", e));
    }

    let screen_state = Arc::new(RwLock::new(ScreenState::new()));
    let mut old_screen_state = ScreenState::new_old();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(32);

    tokio::spawn(async move {
        loop {
            let current_time = Local::now().format("%H:%M").to_string();

            {
                let mut screen_state = screen_state.write().await;
                screen_state.time = current_time;

                loop {
                    if let Ok(msg) = moonraker_rx.lock().await.try_recv() {
                        if let MoonrakerMsg::MsgMethodParam {
                            jsonrpc: _,
                            method: _,
                            params,
                        } = msg.clone()
                        {
                            if let moonraker_api::MoonrakerParam::NotifyStatusUpdate(data, _) =
                                params
                            {
                                if let Some(display_status) = data.get("display_status") {
                                    if let Some(progress) = display_status.get("progress") {
                                        screen_state.printing_progress =
                                            (progress.as_f64().unwrap_or(0.0) * 100.0).round()
                                                as i16;
                                    }
                                }

                                if let Some(extruder) = data.get("extruder") {
                                    if let Some(temperature) = extruder.get("temperature") {
                                        screen_state.nozzle_temp =
                                            temperature.as_f64().unwrap_or(0.0).round() as i16;
                                    }

                                    if let Some(target) = extruder.get("target") {
                                        screen_state.target_nozzle_temp =
                                            target.as_f64().unwrap_or(0.0).round() as i16;
                                    }
                                }

                                if let Some(heater_bed) = data.get("heater_bed") {
                                    if let Some(temperature) = heater_bed.get("temperature") {
                                        screen_state.bed_temp =
                                            temperature.as_f64().unwrap_or(0.0).round() as i16;
                                    }

                                    if let Some(target) = heater_bed.get("target") {
                                        screen_state.target_bed_temp =
                                            target.as_f64().unwrap_or(0.0).round() as i16;
                                    }
                                }

                                //println!("Got status update: {:?}", data);
                            }
                        }

                        if let MoonrakerMsg::MsgResult {
                            jsonrpc: _,
                            result,
                            id,
                        } = msg
                        {
                            if id
                                != moonraker_api::get_method_id(
                                    &moonraker_api::MoonrakerMethod::PrinterObjectsSubscribe,
                                )
                            {
                                continue;
                            }

                            let result: PrinterStateResult = serde_json::from_value(result)
                                .map_err(|e| anyhow::anyhow!("SERDE Error: {}", e))
                                .unwrap();

                            screen_state.printing_progress =
                                (result.status.display_status.progress * 100.0).round() as i16;

                            screen_state.nozzle_temp =
                                result.status.extruder.temperature.round() as i16;
                            screen_state.target_nozzle_temp =
                                result.status.extruder.target.round() as i16;

                            screen_state.bed_temp =
                                result.status.heater_bed.temperature.round() as i16;
                            screen_state.target_bed_temp =
                                result.status.heater_bed.target.round() as i16;

                            //println!("Got result ({}): {:?}", id, result);
                        }
                    } else {
                        break;
                    }
                }

                let update_screen_res = screen_state
                    .update_changed(&mut old_screen_state, &tx)
                    .await;

                if let Err(e) = update_screen_res {
                    println!("Error while updating screen: {}", e);
                }
            }

            /*
                let printer_stats = client.get(format!("http://{}/printer/objects/query?heater_bed=target,temperature&extruder=target,temperature&display_status&print_stats", MOONRAKER_API_URL))
                    .send().await;

                if let Ok(printer_stats) = printer_stats {
                    if let Ok(printer_stats) = printer_stats.json::<PrinterStatsRoot>().await {
                        _ = tx
                            .send(construct_i16(
                                0x2025,
                                printer_stats.result.status.extruder.temperature.round() as i16,
                            ))
                            .await;

                        _ = tx
                            .send(construct_i16(
                                0x2026,
                                printer_stats.result.status.extruder.target as i16,
                            ))
                            .await;

                        _ = tx
                            .send(construct_i16(
                                0x2027,
                                printer_stats.result.status.heater_bed.temperature.round() as i16,
                            ))
                            .await;

                        _ = tx
                            .send(construct_i16(
                                0x2028,
                                printer_stats.result.status.heater_bed.target as i16,
                            ))
                            .await;

                        _ = tx
                            .send(construct_i16(
                                0x2029,
                                (printer_stats.result.status.display_status.progress * 100.0) as i16,
                            ))
                            .await;

                        let mut model_name = printer_stats.result.status.print_stats.filename.clone();
                        if model_name.len() > 20 {
                            model_name = model_name[..20].to_string();
                        } else {
                            let left_pad = (20 - model_name.len()) / 2;
                            let right_pad = 20 - model_name.len() - left_pad;

                            model_name = format!(
                                "{}{}{}",
                                " ".repeat(left_pad),
                                model_name,
                                " ".repeat(right_pad)
                            );
                        }

                        _ = tx.send(construct_text(0x2015, &model_name)).await;

                        let file_metadata = client
                            .get(format!(
                                "http://{}/server/files/metadata?filename={}",
                                MOONRAKER_API_URL, printer_stats.result.status.print_stats.filename
                            ))
                            .send()
                            .await;

                        if let Ok(file_metadata) = file_metadata {
                            if let Ok(file_metadata) = file_metadata.json::<FileMetadataRoot>().await {
                                let est_print_time =
                                    printer_stats.result.status.display_status.progress
                                        * file_metadata.result.estimated_time as f64;

                                let eta = file_metadata.result.estimated_time - est_print_time as i64;
                                let eta_hours = eta / 3600;
                                let eta_minutes = (eta - eta_hours * 3600) / 60;

                                _ = tx
                                    .send(construct_text(
                                        0x2005,
                                        &format!("ETA: {:0>2}:{:0>2}", eta_hours, eta_minutes),
                                    ))
                                    .await;
                            }
                        }
                    }
                }
            */

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let mut serial = serial?;
    check_boot_state(&mut serial).await?;

    let mut last_alive = Instant::now();

    let mut buffer = vec![0; 1024];
    loop {
        if last_alive.elapsed().as_millis() > TIMEOUT_THRESHOLD {
            println!("Connection to screen lost.");
            return Err(anyhow::anyhow!("Connection to screen lost."));
        }

        let len = serial.read(&mut buffer)?;

        if len > 0 {
            last_alive = Instant::now();

            if buffer[3] == 0x82 {
                //ACK when writing
            } else if buffer[3] == 0x83 {
                let address = u16::from_be_bytes([buffer[4], buffer[5]]);
                let data_length = buffer[6] * 2; // word = 2 bytes

                match address {
                    0x14 => {
                        let page_number = u16::from_be_bytes([buffer[7], buffer[8]]);
                        if page_number == 0 {
                            serial.write(&construct_change_page(1))?;
                        }
                    }
                    0x1000 => {
                        let btn = u16::from_be_bytes([buffer[7], buffer[8]]);

                        match btn {
                            2 => {
                                println!("EMERGENCY STOP");
                            }
                            9 => {
                                println!("Restarting firmware...");
                            }
                            _ => {
                                println!("Button pressed: {}", btn);
                            }
                        }
                    }
                    _ => {
                        if data_length > 2 {
                            let value =
                                std::str::from_utf8(&buffer[7..(7 + data_length as usize)])?;
                            println!("Address: {:#X} Value: {}", address, value);
                        } else {
                            let value = u16::from_be_bytes([buffer[7], buffer[8]]);
                            println!("Address: {:#X} Value: {}", address, value);
                        }
                    }
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        if let Ok(data) = rx.try_recv() {
            serial.write(&data)?;
        }
    }

    //Ok(())
}

async fn check_boot_state(serial: &mut Uart) -> Result<()> {
    serial.write(&construct_change_page(1))?;
    let now = Instant::now();

    let mut buffer = vec![0; 1024];
    loop {
        if now.elapsed().as_millis() > BOOT_TIMEOUT {
            _ = serial.write(&construct_change_page(0));
            return Err(anyhow::anyhow!("Connection Timeout"));
        }

        let len = serial.read(&mut buffer)?;
        if len >= 3 {
            if buffer[3] == 0x82 {
                println!("Screen is ready");
                return Ok(());
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}
