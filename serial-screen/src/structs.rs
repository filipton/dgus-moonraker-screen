use crate::serial_utils::{construct_i16, construct_text};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// TODO: maybe use a macro for this?
//       macro should be like serde renaming etc
//       so for each field we can specify the address
//       and page number (it would only send the data if
//                        current page is equals to the page number)
#[derive(Debug, Clone)]
pub struct ScreenState {
    pub current_page: u8,

    pub time: String,           // 0x2000/5 HH:MM
    pub estimated_time: String, // 0x2005/10 ETA: HH:MM

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
        &self,
        old: &mut Self,
        serial_tx: &tokio::sync::mpsc::Sender<Vec<u8>>,
    ) -> Result<()> {
        // always send time because it's like ping
        let _ = serial_tx.send(construct_text(0x2000, &self.time)).await;

        if self.estimated_time != old.estimated_time {
            println!("chg: estimated_time: {}", self.estimated_time);
            let _ = serial_tx
                .send(construct_text(0x2005, &self.estimated_time))
                .await;

            old.estimated_time = self.estimated_time.clone();
        }

        if self.model_name != old.model_name {
            println!("chg: model_name: {}", self.model_name);
            let _ = serial_tx
                .send(construct_text(0x2015, &self.model_name))
                .await;

            old.model_name = self.model_name.clone();
        }

        if self.nozzle_temp != old.nozzle_temp {
            println!("chg: nozzle_temp: {}", self.nozzle_temp);
            let _ = serial_tx
                .send(construct_i16(0x2025, self.nozzle_temp))
                .await;

            old.nozzle_temp = self.nozzle_temp;
        }

        if self.target_nozzle_temp != old.target_nozzle_temp {
            println!("chg: target_nozzle_temp: {}", self.target_nozzle_temp);
            let _ = serial_tx
                .send(construct_i16(0x2026, self.target_nozzle_temp))
                .await;

            old.target_nozzle_temp = self.target_nozzle_temp;
        }

        if self.bed_temp != old.bed_temp {
            println!("chg: bed_temp: {}", self.bed_temp);
            let _ = serial_tx.send(construct_i16(0x2027, self.bed_temp)).await;

            old.bed_temp = self.bed_temp;
        }

        if self.target_bed_temp != old.target_bed_temp {
            println!("chg: target_bed_temp: {}", self.target_bed_temp);
            let _ = serial_tx
                .send(construct_i16(0x2028, self.target_bed_temp))
                .await;

            old.target_bed_temp = self.target_bed_temp;
        }

        if self.printing_progress != old.printing_progress {
            println!("chg: printing_progress: {}", self.printing_progress);
            let _ = serial_tx
                .send(construct_i16(0x2029, self.printing_progress))
                .await;

            old.printing_progress = self.printing_progress;
        }

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrinterStateResult {
    pub eventtime: f64,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "display_status")]
    pub display_status: DisplayStatus,
    pub extruder: Extruder,
    #[serde(rename = "heater_bed")]
    pub heater_bed: HeaterBed,
    #[serde(rename = "print_stats")]
    pub print_stats: PrintStats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayStatus {
    pub progress: f64,
    pub message: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extruder {
    pub target: f64,
    pub temperature: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaterBed {
    pub target: f64,
    pub temperature: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrintStats {
    pub filename: String,
    #[serde(rename = "total_duration")]
    pub total_duration: f64,
    #[serde(rename = "print_duration")]
    pub print_duration: f64,
    #[serde(rename = "filament_used")]
    pub filament_used: f64,
    pub state: String,
    pub message: String,
    pub info: Info,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(rename = "total_layer")]
    pub total_layer: Option<i64>,
    #[serde(rename = "current_layer")]
    pub current_layer: Option<i64>,
}
