use anyhow::Result;
use moonraker_api::{websocket, MoonrakerMsg};

#[tokio::main]
async fn main() -> Result<()> {
    let tx = websocket::connect("192.168.1.18:7125").await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        tx.send(MoonrakerMsg::new(
            moonraker_api::methods::MoonrakerMethod::EmergencyStop,
            None,
        ))?;
    }
}
