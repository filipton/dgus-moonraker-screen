use anyhow::Result;

fn main() -> Result<()> {
    let serial_res = serialport::new("/dev/AMA0", 115200).open();
    if serial_res.is_err() {
        println!("Error opening serial port: {}", serial_res.err().unwrap());
        println!("List of available ports:");

        for port in serialport::available_ports().unwrap() {
            println!("  {}", port.port_name);
        }

        return Ok(());
    }

    Ok(())
}
