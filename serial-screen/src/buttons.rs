use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{mpsc::UnboundedSender, Mutex, RwLock};

use crate::{
    moonraker::{MoonrakerTx, PrinterState},
    screen_state::ScreenState,
    serial_utils::construct_change_page,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Undefined(u16),

    PrintingProgress,
    EmergencyStop,
    Preheat,
    PrinterMovement,
    Settings,
    BackToMain,
    Pause,
    Stop,
    EmergencyStopRelease,
    PreheatPla,
    PreheatCooldown,
    PreheatCustom,
    MacrosUP,
    MacrosDOWN,
    MacrosButton1,
    MacrosButton2,
    MacrosButton3,
    MacrosButton4,
    Macros,
}

impl Button {
    pub fn from_id(id: u16) -> Self {
        match id {
            1 => Button::PrintingProgress,
            2 => Button::EmergencyStop,
            3 => Button::Preheat,
            4 => Button::PrinterMovement,
            5 => Button::Settings,
            6 => Button::BackToMain,
            7 => Button::Pause,
            8 => Button::Stop,
            9 => Button::EmergencyStopRelease,
            10 => Button::PreheatPla,
            11 => Button::PreheatCooldown,
            12 => Button::PreheatCustom,
            13 => Button::MacrosUP,
            14 => Button::MacrosDOWN,
            15 => Button::MacrosButton1,
            16 => Button::MacrosButton2,
            17 => Button::MacrosButton3,
            18 => Button::MacrosButton4,
            19 => Button::Macros,
            _ => Button::Undefined(id),
        }
    }
}

pub async fn parse_button_click(
    button: Button,
    moonraker_tx: &MoonrakerTx,
    screen_state: &Arc<RwLock<ScreenState>>,
    serial_tx: &Arc<Mutex<UnboundedSender<Vec<u8>>>>,
) -> Result<()> {
    let serial_tx = serial_tx.lock().await;
    let moonraker_tx = moonraker_tx.lock().await;
    let mut screen_state = screen_state.write().await;

    match button {
        Button::PrintingProgress => {
            serial_tx.send(construct_change_page(2))?;
        }
        Button::EmergencyStop => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_with_method_and_id(
                moonraker_api::MoonrakerMethod::EmergencyStop,
            ))?;

            serial_tx.send(construct_change_page(3))?;
        }
        Button::Preheat => {
            serial_tx.send(construct_change_page(4))?;
        }
        Button::PrinterMovement => {
            if screen_state.printer_state != PrinterState::Printing
                && screen_state.printer_state != PrinterState::Paused
            {
                serial_tx.send(construct_change_page(5))?;
                return Ok(());
            }
        }
        Button::Settings => {
            //serial_tx.send(construct_change_page(7))?;
        }
        Button::BackToMain => {
            serial_tx.send(construct_change_page(1))?;
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
            serial_tx.send(construct_change_page(1))?;
        }
        Button::PreheatCooldown => {
            if screen_state.printer_state == PrinterState::Printing
                || screen_state.printer_state == PrinterState::Paused
            {
                return Ok(());
            }

            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "TURN_OFF_HEATERS".to_string(),
                },
            ))?;
            serial_tx.send(construct_change_page(1))?;
        }
        Button::PreheatCustom => {}
        Button::MacrosUP => {
            if screen_state.macros_scroll > 0 {
                screen_state.macros_scroll -= 1;
                screen_state.update_macros_list(&serial_tx).await?;
            }
        }
        Button::MacrosDOWN => {
            if screen_state.macros_scroll < screen_state.macros.len() - 1 {
                screen_state.macros_scroll += 1;
                screen_state.update_macros_list(&serial_tx).await?;
            }
        }
        Button::MacrosButton1 => {
            let macro_value = screen_state.macros.get(screen_state.macros_scroll);

            if let Some(macro_value) = macro_value {
                moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                    moonraker_api::MoonrakerMethod::GcodeScript,
                    moonraker_api::MoonrakerParam::GcodeScript {
                        script: macro_value.to_string(),
                    },
                ))?;

                serial_tx.send(construct_change_page(1))?;
            }
        }
        Button::MacrosButton2 => {
            let macro_value = screen_state.macros.get(screen_state.macros_scroll + 1);

            if let Some(macro_value) = macro_value {
                moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                    moonraker_api::MoonrakerMethod::GcodeScript,
                    moonraker_api::MoonrakerParam::GcodeScript {
                        script: macro_value.to_string(),
                    },
                ))?;

                serial_tx.send(construct_change_page(1))?;
            }
        }
        Button::MacrosButton3 => {
            let macro_value = screen_state.macros.get(screen_state.macros_scroll + 2);

            if let Some(macro_value) = macro_value {
                moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                    moonraker_api::MoonrakerMethod::GcodeScript,
                    moonraker_api::MoonrakerParam::GcodeScript {
                        script: macro_value.to_string(),
                    },
                ))?;

                serial_tx.send(construct_change_page(1))?;
            }
        }
        Button::MacrosButton4 => {
            let macro_value = screen_state.macros.get(screen_state.macros_scroll + 3);

            if let Some(macro_value) = macro_value {
                moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                    moonraker_api::MoonrakerMethod::GcodeScript,
                    moonraker_api::MoonrakerParam::GcodeScript {
                        script: macro_value.to_string(),
                    },
                ))?;

                serial_tx.send(construct_change_page(1))?;
            }
        }
        Button::Macros => {
            serial_tx.send(construct_change_page(6))?;
        }
        Button::Undefined(id) => {
            println!("Undefined button pressed with ID: {}", id);
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementButton {
    Undefined(u16),

    ZPlus,
    ZMinus,
    YPlus,
    YMinus,
    XPlus,
    XMinus,
    Home,
}

impl MovementButton {
    pub fn from_id(id: u16) -> Self {
        match id {
            1 => MovementButton::YPlus,
            2 => MovementButton::XPlus,
            3 => MovementButton::XMinus,
            4 => MovementButton::YMinus,
            5 => MovementButton::ZPlus,
            6 => MovementButton::ZMinus,
            7 => MovementButton::Home,
            _ => MovementButton::Undefined(id),
        }
    }
}

pub async fn parse_movement_button(
    button: MovementButton,
    moonraker_tx: &MoonrakerTx,
    screen_state: &Arc<RwLock<ScreenState>>,
    serial_tx: &Arc<Mutex<UnboundedSender<Vec<u8>>>>,
) -> Result<()> {
    let moonraker_tx = moonraker_tx.lock().await;
    let screen_state = screen_state.read().await;

    match button {
        MovementButton::Home => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "G28".to_string(),
                },
            ))?;
        }
        MovementButton::XPlus => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "G91\nG1 X+10 F6000\nG90".to_string(),
                },
            ))?;
        }
        MovementButton::XMinus => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "G91\nG1 X-10 F6000\nG90".to_string(),
                },
            ))?;
        }
        MovementButton::YPlus => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "G91\nG1 Y+10 F6000\nG90".to_string(),
                },
            ))?;
        }
        MovementButton::YMinus => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "G91\nG1 Y-10 F6000\nG90".to_string(),
                },
            ))?;
        }
        MovementButton::ZPlus => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "G91\nG1 Z+10 F6000\nG90".to_string(),
                },
            ))?;
        }
        MovementButton::ZMinus => {
            moonraker_tx.send(moonraker_api::MoonrakerMsg::new_param_id(
                moonraker_api::MoonrakerMethod::GcodeScript,
                moonraker_api::MoonrakerParam::GcodeScript {
                    script: "G91\nG1 Z-10 F6000\nG90".to_string(),
                },
            ))?;
        }
        MovementButton::Undefined(id) => {
            println!("Undefined movement button pressed with ID: {}", id);
        }
    }

    Ok(())
}
