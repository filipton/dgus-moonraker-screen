use anyhow::Result;
use chrono::Local;
use moonraker_api::{MoonrakerMethod, MoonrakerMsg};
use rppal::uart::Uart;
use screen_state::ScreenState;
use serial_utils::construct_change_page;
use std::sync::Arc;
use structs::{FileMetadataRoot, PrinterStateRoot};
use tokio::{
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        Mutex, RwLock,
    },
    time::Instant,
};
use utils::subscribe_websocket_events;

mod moonraker;
mod screen_state;
mod serial_utils;
mod structs;
mod utils;

const RETRY_TIMEOUT: u64 = 5000;
const BOOT_TIMEOUT: u128 = 1000;
const TIMEOUT_THRESHOLD: u128 = 2000;

// TODO: make this configurable (or just hardcode it as localhost)
pub const MOONRAKER_API_URL: &str = "192.168.1.18:7125";

#[tokio::main]
async fn main() -> Result<()> {
    let screen_state = Arc::new(RwLock::new(ScreenState::new()));
    let (tx, rx) = moonraker_api::connect(MOONRAKER_API_URL).await?;
    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));

    subscribe_websocket_events(tx.clone()).await?;
    loop {
        let res = connect_to_serial(screen_state.clone(), tx.clone(), rx.clone()).await;
        if let Err(_) = res {
            // retry after 5 seconds
            tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_TIMEOUT)).await;
        }
    }
}

async fn connect_to_serial(
    screen_state: Arc<RwLock<ScreenState>>,
    moonraker_tx: Arc<Mutex<UnboundedSender<MoonrakerMsg>>>,
    moonraker_rx: Arc<Mutex<UnboundedReceiver<MoonrakerMsg>>>,
) -> Result<()> {
    let serial = rppal::uart::Uart::new(115200, rppal::uart::Parity::None, 8, 1);
    if let Err(e) = serial {
        return Err(anyhow::anyhow!("Error: {}", e));
    }

    let client = reqwest::Client::new();

    let mut old_screen_state = ScreenState::new_old();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(32);

    let screen_update_task = tokio::spawn(async move {
        loop {
            {
                let mut screen_state = screen_state.write().await;
                let current_time = Local::now().format("%H:%M").to_string();
                screen_state.time = current_time;

                let moonraker_update_res = moonraker::recieve_moonraker_updates(
                    &mut screen_state,
                    moonraker_tx.clone(),
                    moonraker_rx.clone(),
                    &client,
                )
                .await;
                if let Err(e) = moonraker_update_res {
                    println!("Error while receiving moonraker updates: {}", e);
                }

                let update_screen_res = screen_state
                    .update_changed(&mut old_screen_state, &tx)
                    .await;

                if let Err(e) = update_screen_res {
                    println!("Error while updating screen: {}", e);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let mut serial = serial?;
    check_boot_state(&mut serial).await?;

    let mut last_alive = Instant::now();

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

                        match btn {
                            2 => {
                                println!("EMERGENCY STOP");
                            }
                            9 => {
                                println!("Restarting firmware...");
                            }
                            _ => {
                                println!("Button pressed: {}", btn);
                            }
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

        if let Ok(data) = rx.try_recv() {
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
        if len >= 3 {
            if buffer[3] == 0x82 {
                println!("Screen is ready");
                return Ok(());
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}
