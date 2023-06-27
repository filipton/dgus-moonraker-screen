use anyhow::Result;

fn main() -> Result<()> {
    let serial = rppal::uart::Uart::new(115200, rppal::uart::Parity::None, 8, 1);
    if let Err(e) = serial {
        println!("Error: {}", e);
        return Ok(());
    }

    let mut serial = serial?;

    let mut buffer = vec![0; 100];
    loop {
        let len = serial.read(&mut buffer)?;
        if len > 0 {
            println!("Read {} bytes: {:#?}", len, &buffer[..len],);
        }
    }
}
