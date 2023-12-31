use crate::{
    screen_state::ScreenState,
    serial_utils::construct_change_page,
    structs::{FileMetadataRoot, PrinterObjectsRoot, PrinterStateRoot},
    utils::{self, subscribe_websocket_events},
};
use anyhow::Result;
use moonraker_api::{MoonrakerMethod, MoonrakerMsg};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex,
};

pub type MoonrakerTx = Arc<Mutex<UnboundedSender<MoonrakerMsg>>>;
pub type MoonrakerRx = Arc<Mutex<UnboundedReceiver<MoonrakerMsg>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum PrinterState {
    Standby,
    Printing,
    Paused,
    Error,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum HomedAxis {
    None,
    X,
    Y,
    XY,
    XYZ,
}

impl From<&str> for PrinterState {
    fn from(s: &str) -> Self {
        match s {
            "standby" => PrinterState::Standby,
            "printing" => PrinterState::Printing,
            "paused" => PrinterState::Paused,
            "error" => PrinterState::Error,
            "complete" => PrinterState::Complete,
            _ => PrinterState::Standby,
        }
    }
}

impl From<&str> for HomedAxis {
    fn from(s: &str) -> Self {
        match s {
            "" => HomedAxis::None,
            "x" => HomedAxis::X,
            "y" => HomedAxis::Y,
            "xy" => HomedAxis::XY,
            "xyz" => HomedAxis::XYZ,
            _ => HomedAxis::None,
        }
    }
}

pub async fn recieve_moonraker_updates(
    screen_state: &mut ScreenState,
    moonraker_tx: &MoonrakerTx,
    moonraker_rx: &MoonrakerRx,
    serial_tx: &Arc<Mutex<UnboundedSender<Vec<u8>>>>,
    client: &reqwest::Client,
    moonraker_api_url: String,
) -> Result<()> {
    while let Ok(msg) = moonraker_rx.lock().await.try_recv() {
        if let MoonrakerMsg::MsgMethodParam {
            jsonrpc: _,
            method: _,
            params,
        } = msg.clone()
        {
            if let moonraker_api::MoonrakerParam::NotifyStatusUpdate(data, _) = params {
                if let Some(display_status) = data.get("display_status") {
                    if let Some(progress) = display_status.get("progress") {
                        screen_state.printing_progress =
                            (progress.as_f64().unwrap_or(0.0) * 100.0).round() as i16;
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
                        screen_state.bed_temp = temperature.as_f64().unwrap_or(0.0).round() as i16;
                    }

                    if let Some(target) = heater_bed.get("target") {
                        screen_state.target_bed_temp =
                            target.as_f64().unwrap_or(0.0).round() as i16;
                    }
                }

                if let Some(print_stats) = data.get("print_stats") {
                    if let Some(filename) = print_stats.get("filename") {
                        let model_name = filename
                            .as_str()
                            .unwrap_or("")
                            .split('.')
                            .next()
                            .unwrap_or("");

                        screen_state.model_name = utils::center_pad(model_name, " ", 20);

                        screen_state.file_estimated_time = get_file_estimated_time(
                            client,
                            filename.as_str().unwrap_or(""),
                            &moonraker_api_url,
                        )
                        .await
                        .unwrap_or(Some(-1))
                        .unwrap_or(-1);
                    }

                    if let Some(state) = print_stats.get("state") {
                        screen_state.printer_state = state.as_str().unwrap_or("").into();
                    }
                }

                if let Some(toolhead) = data.get("toolhead") {
                    if let Some(homed_axes) = toolhead.get("homed_axes") {
                        screen_state.homed_axes = homed_axes.as_str().unwrap_or("").into();
                    }
                }
            }
        }

        if let MoonrakerMsg::MsgResult {
            jsonrpc: _,
            result,
            id,
        } = msg.clone()
        {
            let method = moonraker_api::methods::get_method_from_id(id);
            if let Err(_) = method {
                continue;
            }

            match method? {
                MoonrakerMethod::PrinterObjectsSubscribe => {
                    let result: PrinterStateRoot = serde_json::from_value(result)
                        .map_err(|e| anyhow::anyhow!("SERDE Error: {}", e))?;

                    screen_state.printing_progress =
                        (result.status.display_status.progress * 100.0).round() as i16;
                    screen_state.printer_state = result.status.print_stats.state.as_str().into();
                    screen_state.homed_axes = result.status.toolhead.homed_axes.as_str().into();

                    screen_state.nozzle_temp = result.status.extruder.temperature.round() as i16;
                    screen_state.target_nozzle_temp = result.status.extruder.target.round() as i16;

                    screen_state.bed_temp = result.status.heater_bed.temperature.round() as i16;
                    screen_state.target_bed_temp = result.status.heater_bed.target.round() as i16;

                    let model_name = result
                        .status
                        .print_stats
                        .filename
                        .split('.')
                        .next()
                        .unwrap_or("");
                    screen_state.model_name = utils::center_pad(model_name, " ", 20);

                    screen_state.file_estimated_time = get_file_estimated_time(
                        client,
                        &result.status.print_stats.filename,
                        &moonraker_api_url,
                    )
                    .await
                    .unwrap_or(Some(-1))
                    .unwrap_or(-1);
                }
                MoonrakerMethod::PrinterObjectsList => {
                    let result: PrinterObjectsRoot = serde_json::from_value(result)
                        .map_err(|e| anyhow::anyhow!("SERDE Error: {}", e))?;

                    let macros = result
                        .objects
                        .into_iter()
                        .filter(|o| o.starts_with("gcode_macro"))
                        .map(|o| o.replace("gcode_macro ", ""))
                        .collect::<Vec<_>>();

                    screen_state.macros = macros;
                }
                _ => {} // Ignore other methods
            }
        }

        if let MoonrakerMsg::MsgMethod { jsonrpc: _, method } = msg {
            if method == MoonrakerMethod::NotifyKlippyReady {
                println!("Klippy is ready, subscribing to printer objects.");
                serial_tx
                    .lock()
                    .await
                    .send(construct_change_page(1))
                    .unwrap();

                _ = subscribe_websocket_events(moonraker_tx.clone()).await;

                _ = moonraker_tx
                    .lock()
                    .await
                    .send(MoonrakerMsg::new_with_method_and_id(
                        MoonrakerMethod::PrinterObjectsList,
                    ));
            }
        }
    }

    Ok(())
}

async fn get_file_estimated_time(
    client: &reqwest::Client,
    filename: &str,
    moonraker_api_url: &str,
) -> Result<Option<i32>> {
    let file_metadata = client
        .get(format!(
            "http://{}/server/files/metadata?filename={}",
            moonraker_api_url, filename
        ))
        .send()
        .await;

    if let Ok(file_metadata) = file_metadata {
        if let Ok(file_metadata) = file_metadata.json::<FileMetadataRoot>().await {
            return Ok(Some(file_metadata.result.estimated_time as i32));
        }
    }

    Ok(None)
}
