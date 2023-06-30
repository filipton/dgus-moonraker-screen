use crate::serial_utils::{construct_i16, construct_text};
use anyhow::Result;
use tokio::sync::mpsc::UnboundedSender;

// TODO: maybe use a macro for this?
//       macro should be like serde renaming etc
//       so for each field we can specify the address
//       and page number (it would only send the data if
//                        current page is equals to the page number)
#[derive(Debug, Clone)]
pub struct ScreenState {
    pub current_page: u8,

    pub time: String,       // 0x2000/5 HH:MM
    estimated_time: String, // 0x2005/10 ETA: HH:MM
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

            time: "00:00".to_string(),
            estimated_time: " ".repeat(10).to_string(),
            file_estimated_time: -1,
            model_name: " ".repeat(20).to_string(),
            nozzle_temp: 0,
            target_nozzle_temp: 0,
            bed_temp: 0,
            target_bed_temp: 0,
            printing_progress: 0,
        }
    }

    pub fn new_old() -> ScreenState {
        ScreenState {
            current_page: 0,

            time: String::new(),
            estimated_time: String::new(),
            file_estimated_time: -1,
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
        serial_tx: &UnboundedSender<Vec<u8>>,
    ) -> Result<()> {
        // always send time because it's like ping
        let _ = serial_tx.send(construct_text(0x2000, &self.time));

        if self.time != old.time {
            old.time = self.time.clone();
        }

        if self.model_name != old.model_name {
            let _ = serial_tx.send(construct_text(0x2015, &self.model_name));

            old.model_name = self.model_name.clone();
        }

        if self.nozzle_temp != old.nozzle_temp {
            let _ = serial_tx.send(construct_i16(0x2025, self.nozzle_temp));

            old.nozzle_temp = self.nozzle_temp;
        }

        if self.target_nozzle_temp != old.target_nozzle_temp {
            let _ = serial_tx.send(construct_i16(0x2026, self.target_nozzle_temp));

            old.target_nozzle_temp = self.target_nozzle_temp;
        }

        if self.bed_temp != old.bed_temp {
            let _ = serial_tx.send(construct_i16(0x2027, self.bed_temp));

            old.bed_temp = self.bed_temp;
        }

        if self.target_bed_temp != old.target_bed_temp {
            let _ = serial_tx.send(construct_i16(0x2028, self.target_bed_temp));

            old.target_bed_temp = self.target_bed_temp;
        }

        if self.file_estimated_time != old.file_estimated_time
            || self.printing_progress != old.printing_progress
        {
            let estimated_time_str = self.get_estimate_string();

            if self.estimated_time != estimated_time_str {
                let _ = serial_tx.send(construct_text(0x2005, &estimated_time_str));

                self.estimated_time = estimated_time_str;
                old.file_estimated_time = self.file_estimated_time;
                old.estimated_time = self.estimated_time.clone();
            }
        }

        if self.printing_progress != old.printing_progress {
            let _ = serial_tx.send(construct_i16(0x2029, self.printing_progress));

            old.printing_progress = self.printing_progress;
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
}
