use slcan_fd::{tokio::CanSocket, NominalBitRate, OperatingMode};
use tokio_serial::SerialPortBuilderExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut port = tokio_serial::new("/dev/ttyACM1", 115_200).open_native_async()?;

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let mut can = CanSocket::new(port);

    can.close().await?;
    can.set_operating_mode(OperatingMode::Silent).await?;
    can.open(NominalBitRate::Rate500Kbit).await?;

    let frame = slcan_fd::Can2Frame::new_data(
        slcan_fd::StandardId::new(0x123).unwrap(),
         &[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    can.send(frame).await?;

    loop {
        match can.read().await {
            Ok(frame) => println!("{:?}", frame),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}