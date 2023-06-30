use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;

use crate::{
    moonraker::{MoonrakerTx, PrinterState},
    screen_state::ScreenState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Undefined(u16),

    EmergencyStop,
    Pause,
    Stop,
    EmergencyStopRelease,
    PreheatPla,
    PreheatCooldown,
}

impl Button {
    pub fn from_id(id: u16) -> Self {
        match id {
            2 => Button::EmergencyStop,
            7 => Button::Pause,
            8 => Button::Stop,
            9 => Button::EmergencyStopRelease,
            10 => Button::PreheatPla,
            11 => Button::PreheatCooldown,
            _ => Button::Undefined(id),
        }
    }
}

pub async fn parse_button_click(
    button: Button,
    moonraker_tx: &MoonrakerTx,
    screen_state: &Arc<RwLock<ScreenState>>,
) -> Result<()> {
    let moonraker_tx = moonraker_tx.lock().await;
    let screen_state = screen_state.read().await;

    match button {
        Button::EmergencyStop => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_with_method_and_id(
                moonraker_api::MoonrakerMethod::EmergencyStop,
            ))?;
        }
        Button::Pause => {
            if screen_state.printer_state == PrinterState::Paused {
                moonraker_tx.send(moonraker_api::MoonrakerMsg::new_with_method_and_id(
                    moonraker_api::MoonrakerMethod::PrintResume,
                ))?;
            } else {
                moonraker_tx.send(moonraker_api::MoonrakerMsg::new_with_method_and_id(
                    moonraker_api::MoonrakerMethod::PrintPause,
                ))?;
            }
        }
        Button::Stop => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_with_method_and_id(
                moonraker_api::MoonrakerMethod::PrintCancel,
            ))?;
        }
        Button::EmergencyStopRelease => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_with_method_and_id(
                moonraker_api::MoonrakerMethod::FirmwareRestart,
            ))?;
        }
        Button::PreheatPla => {
            if screen_state.printer_state == PrinterState::Printing
                || screen_state.printer_state == PrinterState::Paused
            {
                // TODO: Maybe popup?

                //serial.write(&construct_change_page(1))?;
                return Ok(());
            }

            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "SET_HEATER_TEMPERATURE HEATER=extruder TARGET=200".to_string(),
                },
            ))?;

            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "SET_HEATER_TEMPERATURE HEATER=heater_bed TARGET=45".to_string(),
                },
            ))?;
        }
        Button::PreheatCooldown => {
            if screen_state.printer_state == PrinterState::Printing
                || screen_state.printer_state == PrinterState::Paused
            {
                // TODO: Maybe popup?

                //serial.write(&construct_change_page(1))?;
                return Ok(());
            }

            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "TURN_OFF_HEATERS".to_string(),
                },
            ))?;
        }
        Button::Undefined(id) => {
            println!("Undefined button pressed with ID: {}", id);
        }
    }

    Ok(())
}
