use std::net::{TcpListener, TcpStream};
use std::io::{Read, Result, Write};
use std::sync::mpsc::Sender;
use std::time::Duration;

use super::{Message, BUFFER_SIZE};

pub struct PeerToPeerConnection {
    client: TcpStream,
    error_tx: Sender<String>
}

impl PeerToPeerConnection {
    pub fn listen_to(addr: &str, timeout: f32, error_tx: Sender<String>) -> Result<Self> {
        let (stream, addr) = TcpListener::bind(addr)?.accept()?;
        stream.set_read_timeout(Some(Duration::from_secs_f32(timeout)))?;
        println!("Connected to peer: {addr}");

        Ok(PeerToPeerConnection { client: stream, error_tx: error_tx })
    }

    pub fn connect_to(addr: &str, timeout: f32, error_tx: Sender<String>) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        stream.set_read_timeout(Some(Duration::from_secs_f32(timeout)))?;
        println!("Connected to peer: {addr}");

        Ok(PeerToPeerConnection { client: stream, error_tx: error_tx })
    }

    pub fn send_message(&mut self, message: Message) -> Result<()> {
        self.client.write_all(&message.to_bytes()).map_err(|err| {
            self.error_tx.send(format!("Error while sending message:\n\n{}", err)).unwrap();
            err
        })
    }

    pub fn wait_for_message(&mut self) -> Option<Message> {
        let mut bytes: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

        match self.client.read_exact(&mut bytes) {
            Ok(_) => {
                match Message::from_bytes(&bytes) {
                    Ok(msg) => Some(msg),
                    Err(_) => {
                        let error_msg = "Invalid message received from peer. Check if both \
                        are running the same game version.".to_string();
                        self.error_tx.send(error_msg).unwrap();
                        None
                    }
                }
            },
            Err(_) => None
        }
    }

    pub fn test_connection(&mut self) -> Result<()> {
        self.send_message(Message::TestConnection())
    }
}
