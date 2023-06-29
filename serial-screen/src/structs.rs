use anyhow::Result;

use crate::serial_utils::construct_text;

#[derive(Debug, Clone)]
pub struct ScreenState {
    pub time: String,           // 0x2000/5 HH:MM
    pub estimated_time: String, // 0x2005/10 ETA: HH:MM

    pub model_name: String, // 0x2015/20 Model Name (centered)

    pub nozzle_temp: i8,        // 0x2025/1
    pub target_nozzle_temp: i8, // 0x2026/1
    pub bed_temp: i8,           // 0x2027/1
    pub target_bed_temp: i8,    // 0x2028/1

    pub printing_progress: i8, // 0x2029/1 (0-100)
}

impl ScreenState {
    pub fn new() -> ScreenState {
        ScreenState {
            time: "00:00".to_string(),
            estimated_time: " ".repeat(10).to_string(),
            model_name: " ".repeat(20).to_string(),
            nozzle_temp: -1,
            target_nozzle_temp: -1,
            bed_temp: -1,
            target_bed_temp: -1,
            printing_progress: -1,
        }
    }

    pub fn new_old() -> ScreenState {
        ScreenState {
            time: String::new(),
            estimated_time: String::new(),
            model_name: String::new(),
            nozzle_temp: 0,
            target_nozzle_temp: 0,
            bed_temp: 0,
            target_bed_temp: 0,
            printing_progress: 0,
        }
    }

    // TODO: maybe create a macro for this?
    pub async fn update_changed(
        &self,
        old: &mut Self,
        serial_tx: &tokio::sync::mpsc::Sender<Vec<u8>>,
    ) -> Result<()> {
        if self.time != old.time {
            let _ = serial_tx.send(construct_text(0x2000, &self.time)).await;
        }

        if self.estimated_time != old.estimated_time {
            let _ = serial_tx
                .send(construct_text(0x2005, &self.estimated_time))
                .await;
        }

        if self.model_name != old.model_name {
            let _ = serial_tx
                .send(construct_text(0x2015, &self.model_name))
                .await;
        }

        if self.nozzle_temp != old.nozzle_temp {
            let _ = serial_tx
                .send(construct_text(0x2025, &self.nozzle_temp.to_string()))
                .await;
        }

        if self.target_nozzle_temp != old.target_nozzle_temp {
            let _ = serial_tx
                .send(construct_text(0x2026, &self.target_nozzle_temp.to_string()))
                .await;
        }

        if self.bed_temp != old.bed_temp {
            let _ = serial_tx
                .send(construct_text(0x2027, &self.bed_temp.to_string()))
                .await;
        }

        if self.target_bed_temp != old.target_bed_temp {
            let _ = serial_tx
                .send(construct_text(0x2028, &self.target_bed_temp.to_string()))
                .await;
        }

        if self.printing_progress != old.printing_progress {
            let _ = serial_tx
                .send(construct_text(0x2029, &self.printing_progress.to_string()))
                .await;
        }

        Ok(())
    }
}
