use anyhow::Result;
use rodio::{source::SineWave, OutputStream, Sink};

use std::net::UdpSocket;

fn main() -> Result<()> {
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let sine = SineWave::new(666.0);

    let connection = UdpSocket::bind("0.0.0.0:0")?;
    connection.connect("192.168.42.1:5001")?;

    let buf = b"Yes, my lord.";

    connection.send(buf)?;

    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        connection.recv(&mut buf)?;

        let Ok(message) = bincode::deserialize::<firmware::Message>(&buf) else {
            continue;
        };
        if let firmware::Message::ButtonReport(message) = message {
            if message.state == firmware::State::Pressed {
                sink.append(sine.clone());
            } else {
                sink.stop();
            }
        }
    }
}
