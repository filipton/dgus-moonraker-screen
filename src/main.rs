use anyhow::Result;

fn main() -> Result<()> {
    let serial = rppal::uart::Uart::new(115200, rppal::uart::Parity::None, 8, 1);
    if let Err(e) = serial {
        println!("Error: {}", e);
        return Ok(());
    }

    let mut serial = serial?;
    serial.write(&construct_send_i16(0x2000, 69))?;

    let mut buffer = vec![0; 100];
    loop {
        let len = serial.read(&mut buffer)?;
        if len > 0 {
            println!("Read {} bytes: {:#?}", len, &buffer[..len],);

            if buffer[3] == 0x82 {
                println!("ACK");
            } else if buffer[3] == 0x83 {
                // read incoming data
                let address = i16::from_be_bytes([buffer[4], buffer[5]]);
                let data_length = buffer[2] - 4;

                if data_length == 2 {
                    let value = i16::from_be_bytes([buffer[7], buffer[8]]);
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

fn construct_send_i16(address: i16, value: i16) -> Vec<u8> {
    let data_length = value.to_be_bytes().len();

    let mut send_buff = vec![
        0x5A,                    // Header
        0xA5,                    // Header
        data_length as u8 + 3u8, // Length (+3 for address and command)
        0x82,                    // Write
    ];
    send_buff.extend_from_slice(&address.to_be_bytes());
    send_buff.extend_from_slice(&value.to_be_bytes());

    send_buff
}

fn construct_send_text(address: i16, value: &str) -> Vec<u8> {
    let data_length = value.len();

    let mut send_buff = vec![
        0x5A,                    // Header
        0xA5,                    // Header
        data_length as u8 + 3u8, // Length (+3 for address and command)
        0x82,                    // Write
    ];
    send_buff.extend_from_slice(&address.to_be_bytes());
    send_buff.extend_from_slice(value.as_bytes());

    send_buff
}
