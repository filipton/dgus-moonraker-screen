use std::time::SystemTime;

use anyhow::Result;
use chrono::{DateTime, Local, Timelike};
use rppal::uart::Uart;
use structs::{FileMetadataRoot, PrinterStatsRoot};
use tokio::time::Instant;
use utils::{construct_change_page, construct_get_page, construct_i16, construct_text};

mod structs;
mod utils;

const RETRY_TIMEOUT: u64 = 5000;
const BOOT_TIMEOUT: u128 = 1000;
const TIMEOUT_CHECK_INTERVAL: u128 = 1000;
const TIMEOUT_THRESHOLD: u128 = 1000;

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        let res = connect_to_serial().await;
        if let Err(_) = res {
            // retry after 5 seconds
            tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_TIMEOUT)).await;
        }
    }
}

async fn connect_to_serial() -> Result<()> {
    let serial = rppal::uart::Uart::new(115200, rppal::uart::Parity::None, 8, 1);
    if let Err(e) = serial {
        return Err(anyhow::anyhow!("Error: {}", e));
    }
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(32);

    tokio::spawn(async move {
        let mut now = Local::now();
        _ = tx
            .send(construct_text(0x2000, &now.format("%H:%M").to_string()))
            .await;

        let client = reqwest::Client::new();
        loop {
            now = Local::now();

            _ = tx
                .send(construct_text(0x2000, &now.format("%H:%M").to_string()))
                .await;

            let printer_stats = client.get("http://192.168.1.18:7125/printer/objects/query?heater_bed=target,temperature&extruder=target,temperature&display_status&print_stats")
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
                            "http://192.168.1.18:7125/server/files/metadata?filename={}",
                            printer_stats.result.status.print_stats.filename
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

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let mut serial = serial?;
    check_boot_state(&mut serial).await?;

    let mut last_alive = Instant::now();
    let mut alive_check_sent = false;

    let mut buffer = vec![0; 1024];
    loop {
        if last_alive.elapsed().as_millis() > TIMEOUT_CHECK_INTERVAL && !alive_check_sent {
            serial.write(&construct_get_page())?;
            alive_check_sent = true;
        } else if last_alive.elapsed().as_millis() > TIMEOUT_CHECK_INTERVAL + TIMEOUT_THRESHOLD {
            println!("Connection to screen lost.");
            return Err(anyhow::anyhow!("Connection to screen lost."));
        }

        let len = serial.read(&mut buffer)?;

        if len > 0 {
            last_alive = Instant::now();
            alive_check_sent = false;
            //println!("Read {} bytes: {:#?}", len, &buffer[..len],);

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
