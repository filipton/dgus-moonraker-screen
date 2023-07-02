use anyhow::Result;
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

    #[serde(rename = "printer.objects.list")]
    PrinterObjectsList,
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
        MoonrakerMethod::PrinterObjectsList => 1454,
        MoonrakerMethod::NotifyKlippyReady => 0,
        MoonrakerMethod::NotifyStatusUpdate => 0,
        MoonrakerMethod::NotifyProcStatUpdate => 0,
    }
}

pub fn get_method_from_id(id: u16) -> Result<MoonrakerMethod> {
    match id {
        3545 => Ok(MoonrakerMethod::FilesMetadata),
        5434 => Ok(MoonrakerMethod::PrinterObjectsSubscribe),
        4564 => Ok(MoonrakerMethod::PrintPause),
        1485 => Ok(MoonrakerMethod::PrintResume),
        2578 => Ok(MoonrakerMethod::PrintCancel),
        4894 => Ok(MoonrakerMethod::PrinterRestart),
        8463 => Ok(MoonrakerMethod::FirmwareRestart),
        //4564 => Ok(MoonrakerMethod::EmergencyStop), // WHY THE FUCK MOONRAKER???
        4645 => Ok(MoonrakerMethod::GcodeScript),
        1454 => Ok(MoonrakerMethod::PrinterObjectsList),
        _ => Err(anyhow::anyhow!("Unknown method id: {}", id)),
    }
}
