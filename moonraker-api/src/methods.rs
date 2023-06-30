use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MoonrakerMethod {
    #[serde(rename = "notify_proc_stat_update")]
    NotifyProcStatUpdate,

    #[serde(rename = "notify_status_update")]
    NotifyStatusUpdate,

    #[serde(rename = "notify_klippy_ready")]
    NotifyKlippyReady,

    #[serde(rename = "printer.objects.subscribe")]
    PrinterObjectsSubscribe,

    #[serde(rename = "server.files.metadata")]
    FilesMetadata,

    #[serde(rename = "printer.print.pause")]
    PrintPause,

    #[serde(rename = "printer.print.resume")]
    PrintResume,

    #[serde(rename = "printer.print.cancel")]
    PrintCancel,

    #[serde(rename = "printer.firmware_restart")]
    FirmwareRestart,

    #[serde(rename = "printer.restart")]
    PrinterRestart,

    #[serde(rename = "printer.emergency_stop")]
    EmergencyStop,

    #[serde(rename = "printer.gcode.script")]
    GcodeScript,
}

pub fn get_method_id(method: &MoonrakerMethod) -> u16 {
    match method {
        MoonrakerMethod::FilesMetadata => 3545,
        MoonrakerMethod::PrinterObjectsSubscribe => 5434,
        MoonrakerMethod::PrintPause => 4564,
        MoonrakerMethod::PrintResume => 1485,
        MoonrakerMethod::PrintCancel => 2578,
        MoonrakerMethod::PrinterRestart => 4894,
        MoonrakerMethod::FirmwareRestart => 8463,
        MoonrakerMethod::EmergencyStop => 4564,
        MoonrakerMethod::GcodeScript => 4645,
        MoonrakerMethod::NotifyKlippyReady => 0,
        MoonrakerMethod::NotifyStatusUpdate => 0,
        MoonrakerMethod::NotifyProcStatUpdate => 0,
    }
}
