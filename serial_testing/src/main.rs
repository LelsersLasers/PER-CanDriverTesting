use std::time::Duration;
use std::io::{self, Read};
use serial::prelude::*;

fn main() -> io::Result<()> {
    println!("Starting...");

    let port_name = "/dev/ttyUSB0"; // MARK

    // Open serial port
    let mut port = serial::open(port_name)?;
    println!("Connected to serial port {}", port_name);
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?; // MARK
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;
    port.set_timeout(Duration::from_millis(100))?;
    println!("Serial port configured");

    let mut buf = [0u8; 64]; // read in up to 64 bytes at a time

    println!("Listening for raw serial data...");

    loop {
        match port.read(&mut buf) {
            Ok(n) if n > 0 => {
                let s = String::from_utf8_lossy(&buf[..n]);
                println!("Received: [{}]", s);
            }
            Ok(_) => {} // no bytes read, continue
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {} // ignore timeout
            Err(e) => {
                eprintln!("Serial read error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
