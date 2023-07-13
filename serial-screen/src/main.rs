use anyhow::Result;
use buttons::{parse_button_click, parse_movement_button, Button, MovementButton};
use moonraker::{MoonrakerRx, MoonrakerTx};
use rppal::uart::Uart;
use screen_state::ScreenState;
use serial_utils::construct_change_page;
use std::sync::Arc;
use tokio::{
    sync::{Mutex, RwLock},
    time::Instant,
};
use updater::check_for_updates;
use utils::subscribe_websocket_events;

mod buttons;
mod moonraker;
mod screen_state;
mod serial_utils;
mod structs;
mod updater;
mod utils;
mod version;

const RETRY_TIMEOUT: u64 = 5000;
const BOOT_TIMEOUT: u128 = 1000;
const TIMEOUT_THRESHOLD: u128 = 2000;

#[tokio::main]
async fn main() -> Result<()> {
    let moonraker_api_url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "localhost:7125".to_string());

    check_for_updates().await;

    let screen_state = Arc::new(RwLock::new(ScreenState::new()));

    let moonraker = moonraker_api::connect(&moonraker_api_url).await?;
    let moonraker_tx = Arc::new(Mutex::new(moonraker.0));
    let moonraker_rx = Arc::new(Mutex::new(moonraker.1));
    subscribe_websocket_events(moonraker_tx.clone()).await?;

    _ = moonraker_tx
        .lock()
        .await
        .send(moonraker_api::MoonrakerMsg::new_with_method_and_id(
            moonraker_api::MoonrakerMethod::PrinterObjectsList,
        ));

    loop {
        let res = connect_to_serial(
            screen_state.clone(),
            moonraker_tx.clone(),
            moonraker_rx.clone(),
            moonraker_api_url.clone(),
        )
        .await;
        if res.is_err() {
            tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_TIMEOUT)).await;
        }
    }
}

async fn connect_to_serial(
    screen_state: Arc<RwLock<ScreenState>>,
    moonraker_tx: MoonrakerTx,
    moonraker_rx: MoonrakerRx,
    moonraker_api_url: String,
) -> Result<()> {
    let serial = rppal::uart::Uart::new(115200, rppal::uart::Parity::None, 8, 1);
    if let Err(e) = serial {
        return Err(anyhow::anyhow!("Serial connection error: {}", e));
    }

    let mut last_alive = Instant::now();
    let mut serial = serial?;
    check_boot_state(&mut serial).await?;

    let (serial_tx, mut serial_rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    let serial_tx = Arc::new(Mutex::new(serial_tx));

    let screen_update_task = screen_state::spawn_update_task(
        moonraker_tx.clone(),
        moonraker_rx.clone(),
        screen_state.clone(),
        serial_tx.clone(),
        moonraker_api_url,
    )
    .await?;

    let mut buffer = vec![0; 1024];
    loop {
        if last_alive.elapsed().as_millis() > TIMEOUT_THRESHOLD {
            println!("Connection to screen lost.");
            screen_update_task.abort();

            return Err(anyhow::anyhow!("Connection to screen lost."));
        }

        let len = serial.read(&mut buffer)?;

        if len > 0 {
            last_alive = Instant::now();

            if buffer[3] == 0x82 {
                //ACK when writing
            } else if buffer[3] == 0x83 {
                let address = u16::from_be_bytes([buffer[4], buffer[5]]);
                let data_length = buffer[6] * 2; // word = 2 bytes

                match address {
                    0x14 => {
                        let page_number = u16::from_be_bytes([buffer[7], buffer[8]]);
                        if page_number == 0 {
                            serial.write(&construct_change_page(1))?;
                        }
                    }
                    0x1000 => {
                        let btn = u16::from_be_bytes([buffer[7], buffer[8]]);
                        let btn = Button::from_id(btn);

                        let res =
                            parse_button_click(btn, &moonraker_tx, &screen_state, &serial_tx).await;
                        if let Err(e) = res {
                            println!("Error while parsing button click: {}", e);
                        }
                    }
                    0x1001 => {
                        let btn = u16::from_be_bytes([buffer[7], buffer[8]]);
                        let btn = MovementButton::from_id(btn);

                        let res =
                            parse_movement_button(btn, &moonraker_tx, &screen_state, &serial_tx)
                                .await;
                        if let Err(e) = res {
                            println!("Error while parsing button click: {}", e);
                        }
                    }
                    _ => {
                        if data_length > 2 {
                            let value =
                                std::str::from_utf8(&buffer[7..(7 + data_length as usize)])?;
                            println!("Address: {:#X} Value: {}", address, value);
                        } else {
                            let value = u16::from_be_bytes([buffer[7], buffer[8]]);
                            println!("Address: {:#X} Value: {}", address, value);
                        }
                    }
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        if let Ok(data) = serial_rx.try_recv() {
            serial.write(&data)?;
        }
    }

    //Ok(())
}

async fn check_boot_state(serial: &mut Uart) -> Result<()> {
    serial.write(&construct_change_page(1))?;
    let now = Instant::now();

    let mut buffer = vec![0; 1024];
    loop {
        if now.elapsed().as_millis() > BOOT_TIMEOUT {
            _ = serial.write(&construct_change_page(0));
            return Err(anyhow::anyhow!("Connection Timeout"));
        }

        let len = serial.read(&mut buffer)?;
        if len >= 3 && buffer[3] == 0x82 {
            println!("Screen is ready");
            return Ok(());
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}
