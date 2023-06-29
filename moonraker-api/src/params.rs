use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MoonrakerParam {
    PrinterObjectsSubscribe {
        objects: HashMap<String, Option<Vec<String>>>,
    },

    NotifyStatusUpdate(HashMap<String, HashMap<String, Value>>, f64),
}
