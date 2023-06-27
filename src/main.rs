use anyhow::Result;

fn main() -> Result<()> {
    let serial = rppal::uart::Uart::new(115200, rppal::uart::Parity::None, 8, 1);
    if let Err(e) = serial {
        println!("Error: {}", e);
        return Ok(());
    }

    let mut serial = serial?;

    /*
    let mut buffer = vec![0; 100];
    loop {
        let len = serial.read(&mut buffer)?;
        if len > 0 {
            println!("Read {} bytes: {:#?}", len, &buffer[..len],);
        }
    }
    */

    let val = 69i16;
    let data_length = val.to_be_bytes().len();

    let mut send_buff = vec![
        0x5A, 0xA5, // Header
        data_length as u8 + 4u8, // Data length
        0x82, // Write
        0x20, // Address
        0x00, // Address
    ];
    send_buff.extend_from_slice(&val.to_be_bytes());

    // id why but sometimes i need to send data twice to change the value
    serial.write(&send_buff)?;

    Ok(())
}
