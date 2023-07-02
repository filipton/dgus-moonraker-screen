use std::sync::Arc;

use crate::{
    moonraker::{self, HomedAxis, MoonrakerRx, MoonrakerTx, PrinterState},
    serial_utils::{construct_change_page, construct_i16, construct_text},
};
use anyhow::Result;
use chrono::Local;
use tokio::{
    sync::{mpsc::UnboundedSender, Mutex, MutexGuard, RwLock},
    task::JoinHandle,
};

// TODO: maybe use a macro for this?
//       macro should be like serde renaming etc
//       so for each field we can specify the address
//       and page number (it would only send the data if
//                        current page is equals to the page number)
#[derive(Debug, Clone)]
pub struct ScreenState {
    pub current_page: u8,
    pub printer_state: PrinterState,
    pub homed_axes: HomedAxis,

    pub macros: Vec<String>,
    pub macros_scroll: usize,

    pub time: String,           // 0x2000/5 HH:MM
    pub estimated_time: String, // 0x2005/10 ETA: HH:MM
    pub file_estimated_time: i32,

    pub model_name: String, // 0x2015/20 Model Name (centered)

    pub nozzle_temp: i16,        // 0x2025/1
    pub target_nozzle_temp: i16, // 0x2026/1
    pub bed_temp: i16,           // 0x2027/1
    pub target_bed_temp: i16,    // 0x2028/1

    pub printing_progress: i16, // 0x2029/1 (0-100)
}

impl ScreenState {
    pub fn new() -> ScreenState {
        ScreenState {
            current_page: 0,
            printer_state: PrinterState::Standby,
            homed_axes: HomedAxis::None,

            macros: Vec::new(),
            macros_scroll: 0,

            time: "00:00".to_string(),
            estimated_time: " ".repeat(10),
            file_estimated_time: -1,
            model_name: " ".repeat(20),
            nozzle_temp: 0,
            target_nozzle_temp: 0,
            bed_temp: 0,
            target_bed_temp: 0,
            printing_progress: 0,
        }
    }

    /// Creates a new ScreenState with old values
    /// It must be different from new() because we want to send
    /// all the data on the first update
    pub fn new_old() -> ScreenState {
        ScreenState {
            current_page: 0,
            printer_state: PrinterState::Paused,
            homed_axes: HomedAxis::XYZ,

            macros: vec!["".into()],
            macros_scroll: 0,

            time: String::new(),
            estimated_time: String::new(),
            file_estimated_time: -2,
            model_name: String::new(),
            nozzle_temp: -1,
            target_nozzle_temp: -1,
            bed_temp: -1,
            target_bed_temp: -1,
            printing_progress: -1,
        }
    }

    // TODO: maybe create a macro for this?
    pub async fn update_changed(
        &mut self,
        old: &mut Self,
        serial_tx: &Arc<Mutex<UnboundedSender<Vec<u8>>>>,
    ) -> Result<()> {
        let serial_tx = serial_tx.lock().await;

        // always send time because it's like ping
        let _ = serial_tx.send(construct_text(0x2000, &self.time));

        if self.time != old.time {
            old.time = self.time.clone();
        }

        if self.model_name != old.model_name {
            _ = serial_tx.send(construct_text(0x2015, &self.model_name));

            old.model_name = self.model_name.clone();
        }

        if self.nozzle_temp != old.nozzle_temp {
            _ = serial_tx.send(construct_i16(0x2025, self.nozzle_temp));

            old.nozzle_temp = self.nozzle_temp;
        }

        if self.target_nozzle_temp != old.target_nozzle_temp {
            _ = serial_tx.send(construct_i16(0x2026, self.target_nozzle_temp));

            old.target_nozzle_temp = self.target_nozzle_temp;
        }

        if self.bed_temp != old.bed_temp {
            _ = serial_tx.send(construct_i16(0x2027, self.bed_temp));

            old.bed_temp = self.bed_temp;
        }

        if self.target_bed_temp != old.target_bed_temp {
            _ = serial_tx.send(construct_i16(0x2028, self.target_bed_temp));

            old.target_bed_temp = self.target_bed_temp;
        }

        if self.file_estimated_time != old.file_estimated_time
            || self.printing_progress != old.printing_progress
        {
            let estimated_time_str = self.get_estimate_string();

            if self.estimated_time != estimated_time_str {
                _ = serial_tx.send(construct_text(0x2005, &estimated_time_str));

                self.estimated_time = estimated_time_str;
                old.file_estimated_time = self.file_estimated_time;
                old.estimated_time = self.estimated_time.clone();
            }
        }

        if self.printing_progress != old.printing_progress {
            _ = serial_tx.send(construct_i16(0x2029, self.printing_progress));

            old.printing_progress = self.printing_progress;
        }

        if self.printer_state != old.printer_state {
            _ = serial_tx.send(construct_i16(
                0x2030,
                (self.printer_state == PrinterState::Paused) as i16,
            ));

            if self.printer_state == PrinterState::Printing {
                // Change page to printing status page
                _ = serial_tx.send(construct_change_page(2));
            } else {
                _ = serial_tx.send(construct_text(0x2005, " ".repeat(10).as_str()));
            }

            old.printer_state = self.printer_state;
        }

        if self.macros != old.macros || self.macros_scroll != old.macros_scroll {
            self.update_macros_list(&serial_tx).await?;

            old.macros = self.macros.clone();
            old.macros_scroll = self.macros_scroll;
        }

        Ok(())
    }

    fn get_estimate_string(&self) -> String {
        if self.file_estimated_time == -1 {
            " ".repeat(10)
        } else {
            let est_print_time =
                (self.printing_progress as f64 / 100.0) * self.file_estimated_time as f64;

            let eta = self.file_estimated_time - est_print_time as i32;
            let eta_hours = eta / 3600;
            let eta_minutes = (eta - eta_hours * 3600) / 60;
            format!("ETA: {:0>2}:{:0>2}", eta_hours, eta_minutes)
        }
    }

    pub async fn update_macros_list(
        &self,
        serial_tx: &MutexGuard<'_, UnboundedSender<Vec<u8>>>,
    ) -> Result<()> {
        let shifted_macros = self
            .macros
            .iter()
            .skip(self.macros_scroll)
            .take(4)
            .map(|x| x.as_str())
            .collect::<Vec<&str>>();

        let mut idx = 0;
        for addr in vec![0x3000, 0x3051, 0x3102, 0x3153] {
            let line_value = shifted_macros.get(idx).unwrap_or(&"");
            let mut line_value = format!("{: <50}", line_value);
            if line_value.len() > 50 {
                line_value.truncate(50);
            }

            let _ = serial_tx.send(construct_text(addr, &line_value));
            idx += 1;
        }

        Ok(())
    }
}

pub async fn spawn_update_task(
    moonraker_tx: MoonrakerTx,
    moonraker_rx: MoonrakerRx,
    screen_state: Arc<RwLock<ScreenState>>,
    serial_tx: Arc<Mutex<UnboundedSender<Vec<u8>>>>,
) -> Result<JoinHandle<()>> {
    let task = tokio::spawn(async move {
        let client = reqwest::Client::new();
        let mut old_screen_state = ScreenState::new_old();

        loop {
            {
                let mut screen_state = screen_state.write().await;
                let current_time = Local::now().format("%H:%M").to_string();
                screen_state.time = current_time;

                let moonraker_update_res = moonraker::recieve_moonraker_updates(
                    &mut screen_state,
                    &moonraker_tx,
                    &moonraker_rx,
                    &serial_tx,
                    &client,
                )
                .await;
                if let Err(e) = moonraker_update_res {
                    println!("Error while receiving moonraker updates: {}", e);
                }

                let update_screen_res = screen_state
                    .update_changed(&mut old_screen_state, &serial_tx)
                    .await;
                if let Err(e) = update_screen_res {
                    println!("Error while updating screen: {}", e);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    Ok(task)
}
