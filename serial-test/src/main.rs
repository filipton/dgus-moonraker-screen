use anyhow::Result;
use rppal::uart::Uart;
use tokio::time::Instant;
use utils::{construct_change_page, construct_i16};

mod utils;

const RETRY_TIMEOUT: u64 = 5000;
const BOOT_TIMEOUT: u128 = 1000;
const TIMEOUT_CHECK_INTERVAL: u128 = 1000;
const TIMEOUT_THRESHOLD: u128 = 1000;

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        let res = connect_to_serial().await;
        if let Err(_) = res {
            // retry after 5 seconds
            tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_TIMEOUT)).await;
        }
    }
}

async fn connect_to_serial() -> Result<()> {
    let serial = rppal::uart::Uart::new(115200, rppal::uart::Parity::None, 8, 1);
    if let Err(e) = serial {
        println!("Error: {}", e);
        return Ok(());
    }

    let mut serial = serial?;
    check_boot_state(&mut serial).await?;

    let mut last_ack = Instant::now();
    let mut ack_sent = false;

    let mut buffer = vec![0; 1024];
    loop {
        if last_ack.elapsed().as_millis() > TIMEOUT_CHECK_INTERVAL && !ack_sent {
            serial.write(&construct_i16(0xFFEE, 69))?;
            ack_sent = true;
        } else if last_ack.elapsed().as_millis() > TIMEOUT_CHECK_INTERVAL + TIMEOUT_THRESHOLD {
            println!("Connection to screen lost.");
            return Ok(());
        }

        let len = serial.read(&mut buffer)?;
        if len > 0 {
            if buffer[3] == 0x82 {
                last_ack = Instant::now();
                ack_sent = false;
            } else if buffer[3] == 0x83 {
                //println!("Read {} bytes: {:#?}", len, &buffer[..len],);

                // read incoming data
                let address = u16::from_be_bytes([buffer[4], buffer[5]]);
                let data_length = buffer[2] - 4;

                if data_length == 2 {
                    let value = u16::from_be_bytes([buffer[7], buffer[8]]);
                    println!("Address: {:#X} Value: {}", address, value);
                } else {
                    let value = std::str::from_utf8(&buffer[7..(7 + data_length as usize)])?;
                    println!("Address: {:#X} Value: {}", address, value);
                }
            }
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
            return Err(anyhow::anyhow!("Connection Timeout"));
        }

        let len = serial.read(&mut buffer)?;
        if len >= 3 {
            if buffer[3] == 0x82 {
                println!("Screen is ready");
                return Ok(());
            }
        }
    }
}
