use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MoonrakerParam {
    PrinterObjectsSubscribe {
        objects: HashMap<String, Option<Vec<String>>>,
    },

    NotifyProcStatUpdate(Vec<NotifyProcStatUpdateRes>),
    NotifyStatusUpdate(HashMap<String, HashMap<String, Value>>, f64),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotifyProcStatUpdateRes {
    pub moonraker_stats: MoonrakerStats,
    pub cpu_temp: f64,
    pub system_cpu_usage: SystemCpuUsage,
    pub system_memory: SystemMemory,
    pub websocket_connections: u64,
    pub network: HashMap<String, NetworkStats>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoonrakerStats {
    pub time: f64,
    #[serde(rename = "cpu_usage")]
    pub cpu_usage: f64,
    pub memory: i64,
    #[serde(rename = "mem_units")]
    pub mem_units: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkStats {
    #[serde(rename = "rx_bytes")]
    pub rx_bytes: i64,
    #[serde(rename = "tx_bytes")]
    pub tx_bytes: i64,
    #[serde(rename = "rx_packets")]
    pub rx_packets: i64,
    #[serde(rename = "tx_packets")]
    pub tx_packets: i64,
    #[serde(rename = "rx_errs")]
    pub rx_errs: i64,
    #[serde(rename = "tx_errs")]
    pub tx_errs: i64,
    #[serde(rename = "rx_drop")]
    pub rx_drop: i64,
    #[serde(rename = "tx_drop")]
    pub tx_drop: i64,
    pub bandwidth: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemCpuUsage {
    pub cpu: f64,
    pub cpu0: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemMemory {
    pub total: i64,
    pub available: i64,
    pub used: i64,
}
