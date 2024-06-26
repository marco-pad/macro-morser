use anyhow::Result;
use rodio::{source::SineWave, Decoder, OutputStream, Sink};

use std::{io::Cursor, net::UdpSocket, thread, time::Duration};

fn main() -> Result<()> {
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let sine = SineWave::new(666.0);

    let fart_data = Cursor::new(include_bytes!("../assets/fart.ogg"));
    let villager = Cursor::new(include_bytes!("../assets/idle2.mp3"));
    let death = Cursor::new(include_bytes!("../assets/death.mp3"));

    let connection = UdpSocket::bind("0.0.0.0:0")?;
    connection.connect("192.168.42.1:5001")?;

    let buf = b"Yes, my lord.";

    connection.send(buf)?;

    ping_thread(connection.try_clone()?);

    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        connection.recv(&mut buf)?;

        let Ok(message) = bincode::deserialize::<firmware::Message>(&buf) else {
            continue;
        };
        if let firmware::Message::ButtonReport(message) = message {
            if message.state == firmware::State::Pressed {
                // let fart = Decoder::new_vorbis(fart_data.clone())?;
                match message.id {
                    2 => {
                        let death = Decoder::new_mp3(death.clone())?;
                        sink.append(death);
                    }
                    3 => {
                        let idle = Decoder::new_mp3(villager.clone())?;
                        sink.append(idle);
                    }
                    4 => {
                        let fart = Decoder::new_vorbis(fart_data.clone())?;
                        sink.append(fart);
                    }
                    _ => {
                        sink.append(sine.clone());
                    }
                }
            } else {
                sink.stop();
            }
        }
    }
}

fn ping_thread(socket: UdpSocket) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        socket
            .send(&bincode::serialize(&firmware::Message::Ping).unwrap())
            .unwrap();
    });
}
