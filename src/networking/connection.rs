use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::{Read, Result, Write, Error, ErrorKind};

use super::Message;

pub struct PeerToPeerConnection {
    client: TcpStream,
    remote_address: SocketAddr
}

impl PeerToPeerConnection {
    pub fn listen_to(addr: &str) -> Result<Self> {
        let (stream, addr) = TcpListener::bind(addr).unwrap().accept()?;
        println!("Connected to peer: {addr}");

        Ok(PeerToPeerConnection { client: stream, remote_address: addr })
    }

    pub fn connect_to(addr: &str) -> Result<Self> {
        let client = TcpStream::connect(addr)?;
        println!("Connected to peer: {addr}");

        let address: SocketAddr = addr.parse().expect("Unable to parse address to a socket.");
        Ok(PeerToPeerConnection { client: client, remote_address: address })
    }

    pub fn send_message(&mut self, message: Message) -> Result<()> {
        self.client.write_all(&message.to_bytes())
    }

    pub fn wait_for_message(&mut self) -> Result<Message> {
        let mut bytes: Vec<u8> = Vec::new();
        self.client.read_to_end(&mut bytes)?;

        Message::from_bytes(&bytes).map_err(|err| {
            Error::new(ErrorKind::InvalidData, err)
        })
    }
}
