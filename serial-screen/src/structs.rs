use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrinterStateRoot {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadataRoot {
    pub result: FileMetadataResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadataResult {
    pub size: i64,
    pub modified: f64,
    pub uuid: String,
    pub slicer: String,
    #[serde(rename = "slicer_version")]
    pub slicer_version: String,
    #[serde(rename = "gcode_start_byte")]
    pub gcode_start_byte: i64,
    #[serde(rename = "gcode_end_byte")]
    pub gcode_end_byte: i64,
    #[serde(rename = "layer_count")]
    pub layer_count: i64,
    #[serde(rename = "object_height")]
    pub object_height: f64,
    #[serde(rename = "estimated_time")]
    pub estimated_time: i64,
    #[serde(rename = "nozzle_diameter")]
    pub nozzle_diameter: f64,
    #[serde(rename = "layer_height")]
    pub layer_height: f64,
    #[serde(rename = "first_layer_height")]
    pub first_layer_height: f64,
    #[serde(rename = "first_layer_extr_temp")]
    pub first_layer_extr_temp: f64,
    #[serde(rename = "first_layer_bed_temp")]
    pub first_layer_bed_temp: f64,
    #[serde(rename = "filament_name")]
    pub filament_name: String,
    #[serde(rename = "filament_type")]
    pub filament_type: String,
    #[serde(rename = "filament_total")]
    pub filament_total: f64,
    #[serde(rename = "filament_weight_total")]
    pub filament_weight_total: f64,
    #[serde(rename = "print_start_time")]
    pub print_start_time: f64,
    #[serde(rename = "job_id")]
    pub job_id: String,
    pub filename: String,
}
