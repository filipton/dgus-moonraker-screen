use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MoonrakerMethod {
    #[serde(rename = "printer.emergency_stop")]
    EmergencyStop,

    #[serde(rename = "notify_proc_stat_update")]
    NotifyProcStatUpdate,

    #[serde(rename = "notify_status_update")]
    NotifyStatusUpdate,

    #[serde(rename = "printer.objects.subscribe")]
    PrinterObjectsSubscribe,
}

pub fn get_method_id(method: &MoonrakerMethod) -> u16 {
    match method {
        MoonrakerMethod::EmergencyStop => 4564,
        MoonrakerMethod::PrinterObjectsSubscribe => 5434,

        MoonrakerMethod::NotifyStatusUpdate => 0,
        MoonrakerMethod::NotifyProcStatUpdate => 0,
    }
}
