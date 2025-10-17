use serial::prelude::*;
use slcan::{BitRate, CanSocket};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    println!("Starting...");

    let port_name = "/dev/ttyACM1"; // MARK

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
    port.set_timeout(Duration::from_millis(200))?;
    println!("Serial port configured");

    // Create the CAN socket
    let mut can_socket = CanSocket::new(port);

    // Set bitrate (adjust to your CAN network’s bitrate)
    can_socket.open(BitRate::Setup500Kbit)?;
    println!("Connected to CAN");

    // Main loop: continuously read frames
    loop {
        match can_socket.read() {
            Ok(frame) => println!("{}", frame),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // No frame available yet, just continue
            }
            Err(e) => {
                eprintln!("Error reading frame: {:?}", e);
                break;
            }
        }
    }

    can_socket.close()?;
    Ok(())
}
