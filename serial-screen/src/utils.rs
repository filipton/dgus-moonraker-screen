use anyhow::Result;
use moonraker_api::MoonrakerMsg;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc::UnboundedSender, Mutex};

// TODO: optimize
pub fn center_pad(s: &str, pad_char: &str, width: usize) -> String {
    if s.len() >= width {
        return s[..width].to_string();
    }

    let l = (width - s.len()) / 2;
    let r = width - s.len() - l;

    format!("{}{}{}", pad_char.repeat(l), s, pad_char.repeat(r))
}

pub async fn subscribe_websocket_events(
    tx: Arc<Mutex<UnboundedSender<MoonrakerMsg>>>,
) -> Result<()> {
    let mut objects: HashMap<String, Option<Vec<String>>> = HashMap::new();
    objects.insert("display_status".to_string(), None);
    objects.insert("print_stats".to_string(), None);
    objects.insert("toolhead".to_string(), None);
    objects.insert(
        "extruder".to_string(),
        Some(vec!["target".into(), "temperature".into()]),
    );
    objects.insert(
        "heater_bed".to_string(),
        Some(vec!["target".into(), "temperature".into()]),
    );

    // subscribe to printer updates
    tx.lock().await.send(MoonrakerMsg::new_param_id(
        moonraker_api::methods::MoonrakerMethod::PrinterObjectsSubscribe,
        moonraker_api::params::MoonrakerParam::PrinterObjectsSubscribe { objects },
    ))?;

    Ok(())
}
