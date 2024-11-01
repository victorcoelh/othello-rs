use std::net::{TcpListener, TcpStream};
use std::io::{Read, Result, Write, Error, ErrorKind};

use super::Message;

pub struct PeerToPeerConnection {
    client: TcpStream,
}

impl PeerToPeerConnection {
    pub fn listen_to(addr: &str) -> Result<Self> {
        let (stream, addr) = TcpListener::bind(addr).unwrap().accept()?;
        println!("Connected to peer: {addr}");

        Ok(PeerToPeerConnection { client: stream })
    }

    pub fn connect_to(addr: &str) -> Result<Self> {
        let client = TcpStream::connect(addr)?;
        println!("Connected to peer: {addr}");

        //let address: SocketAddr = addr.parse().expect("Unable to parse address to a socket.");
        Ok(PeerToPeerConnection { client: client })
    }

    pub fn send_message(&mut self, message: Message) -> Result<()> {
        self.client.write_all(&message.to_bytes())
    }

    pub fn wait_for_message(&mut self) -> Result<Message> {
        self.client.set_nonblocking(false).unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        self.client.read_to_end(&mut bytes)?;

        Message::from_bytes(&bytes).map_err(|err| {
            Error::new(ErrorKind::InvalidData, err)
        })
    }

    pub fn get_message_if_available(&mut self) -> Option<Message> {
        self.client.set_nonblocking(true).unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        match self.client.read_to_end(&mut bytes) {
            Ok(_) => {
                Some(Message::from_bytes(&bytes).expect("Received invalid data."))
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => None,
            Err(e) => panic!("IO Error when reading from connection: {e}")
        }
    }
}
