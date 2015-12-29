use std::thread;
use std::str;
use std::io::Read;
use std::net::TcpStream;
use ircrs::config::Config;
use ircrs::room::Room;

pub struct Session {
    stream: TcpStream,
    config: Config,
    rooms: Vec<Room>
}

impl Session {
    pub fn new(stream: TcpStream, config: Config) -> Session {
        return Session {
            stream: stream,
            config: config,
            rooms:  Vec::new()
        };
    }

    pub fn run(&mut self) {

        let mut buf  = [0u8; 256];
        while let Ok(n) = self.stream.read(&mut buf) {
            println!("{}", str::from_utf8(&buf).unwrap());
        }

        let mut reader = self.stream.try_clone().unwrap();
        thread::spawn(move || {
            let mut buf  = [0u8; 256];
            while let Ok(n) = reader.read(&mut buf) {
                println!("{}", str::from_utf8(&buf).unwrap());
            }
        });
    }
}
