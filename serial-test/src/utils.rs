pub fn construct_change_page(page_number: u16) -> Vec<u8> {
    let mut send_buff = vec![
        0x5A, // Header
        0xA5, // Header
        0x07, // Length (4 (page number) + 3 (address and command))
        0x82, // Write
        0x00, // Address Page Reg
        0x84, // Address Page Reg
        0x5A, // Data Header
        0x01, // Data Header
    ];
    send_buff.extend_from_slice(&page_number.to_be_bytes());

    send_buff
}

pub fn construct_i16(address: u16, value: i16) -> Vec<u8> {
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

pub fn construct_text(address: u16, value: &str) -> Vec<u8> {
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
