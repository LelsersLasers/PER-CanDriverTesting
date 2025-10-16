use socketcan::{CanFrame, CanSocket, EmbeddedFrame, Socket};
use std::io;

fn main() -> io::Result<()> {
    let iface = "can0";

    let socket = CanSocket::open(iface)?;
    println!("Listening on CAN interface {}", iface);

    loop {
        match socket.read_frame() {
            Ok(frame) => print_frame(&frame),
            Err(e) => eprintln!("Error reading CAN frame: {:?}", e),
        }
    }
}

fn print_frame(frame: &CanFrame) {
    let id = match frame.id() {
        socketcan::Id::Standard(id) => id.as_raw() as u32,
        socketcan::Id::Extended(id) => id.as_raw(),
    };
    let data = frame.data();
    let dlc = frame.data().len();

    println!(
        "CANFrame {{ id: {:X}, dlc: {}, data: {:?} }}",
        id, dlc, data
    );
}
