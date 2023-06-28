pub fn construct_change_page(page_number: u16) -> Vec<u8> {
    let mut page_number_buffer = vec![0x5A, 0x01]; // Data header (idk what this is for)
    page_number_buffer.extend_from_slice(&page_number.to_be_bytes());

    construct_write_buf(0x0084, &page_number_buffer)
}

pub fn construct_i16(address: u16, value: i16) -> Vec<u8> {
    construct_write_buf(address, &value.to_be_bytes())
}

pub fn construct_text(address: u16, value: &str) -> Vec<u8> {
    construct_write_buf(address, value.as_bytes())
}

pub fn construct_write_buf(address: u16, buffer: &[u8]) -> Vec<u8> {
    let data_length = buffer.len();

    let mut send_buff = vec![
        0x5A,                    // Header
        0xA5,                    // Header
        data_length as u8 + 3u8, // Length (+3 for address and command)
        0x82,                    // Write
    ];
    send_buff.extend_from_slice(&address.to_be_bytes());
    send_buff.extend_from_slice(buffer);

    send_buff
}
