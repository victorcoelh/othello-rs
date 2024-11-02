use std::net::{TcpListener, TcpStream};
use std::io::{Read, Result, Write, Error, ErrorKind};
use std::time::Duration;

use super::Message;

pub struct PeerToPeerConnection {
    client: TcpStream
}

impl PeerToPeerConnection {
    pub fn listen_to(addr: &str, timeout: f32) -> Result<Self> {
        let (stream, addr) = TcpListener::bind(addr).unwrap().accept()?;
        stream.set_read_timeout(Some(Duration::from_secs_f32(timeout))).unwrap();
        println!("Connected to peer: {addr}");

        Ok(PeerToPeerConnection { client: stream })
    }

    pub fn connect_to(addr: &str, timeout: f32) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        stream.set_read_timeout(Some(Duration::from_secs_f32(timeout))).unwrap();
        println!("Connected to peer: {addr}");

        //let address: SocketAddr = addr.parse().expect("Unable to parse address to a socket.");
        Ok(PeerToPeerConnection { client: stream })
    }

    pub fn send_message(&mut self, message: Message) -> Result<()> {
        println!("sent message");
        self.client.write_all(&message.to_bytes())
    }

    pub fn wait_for_message(&mut self) -> Option<Message> {
        let mut bytes: Vec<u8> = Vec::new();
        match self.client.read_to_end(&mut bytes) {
            Ok(0) => None,
            Ok(_) => {
                println!("received message: {:?}", bytes);
                let message = Message::from_bytes(&bytes).map_err(|err| {
                    Error::new(ErrorKind::InvalidData, err)
                }).unwrap();
                Some(message)
            },
            Err(_) => None
        }
    }
}
