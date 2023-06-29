use std::collections::HashMap;

use anyhow::Result;
use moonraker_api::{methods::MoonrakerMethod, params::MoonrakerParam, websocket, MoonrakerMsg};

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = websocket::connect("192.168.1.18:7125").await?;

    /*
    let mut objects: HashMap<String, Option<Vec<String>>> = HashMap::new();
    objects.insert("display_status".to_string(), None);
    objects.insert("print_stats".to_string(), None);
    objects.insert(
        "extruder".to_string(),
        Some(vec!["target".into(), "temperature".into()]),
    );
    objects.insert(
        "heater_bed".to_string(),
        Some(vec!["target".into(), "temperature".into()]),
    );

    tx.send(MoonrakerMsg::new(
        moonraker_api::methods::MoonrakerMethod::PrinterObjectsSubscribe,
        Some(moonraker_api::params::MoonrakerParam::PrinterObjectsSubscribe { objects }),
    ))?;
    */

    tx.send(MoonrakerMsg::new_param_id(
        MoonrakerMethod::FilesMetadata,
        MoonrakerParam::FilesMetadata {
            filename: "bcut-stk.gcode".to_string(),
        },
    ))?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        if let Some(msg) = rx.recv().await {
            println!("DBG: Received: {:#?}", msg);
        }
    }
}
