use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MoonrakerMethod {
    #[serde(rename = "printer.emergency_stop")]
    EmergencyStop,
}

pub fn get_method_id(method: &MoonrakerMethod) -> u16 {
    match method {
        MoonrakerMethod::EmergencyStop => 4564,
    }
}
