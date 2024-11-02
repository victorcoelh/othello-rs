use crate::game_logic::OthelloBoard;
use crate::networking::{Message, PeerToPeerConnection};
use std::io::Error;
use std::sync::mpsc;
use std::thread;

pub enum GameState {
    NoConnection,
    Playing,
    GameEnded,
}

pub struct GameController {
    state: GameState,
    board: OthelloBoard,
    chat_messages: Vec<String>,
    controller_tx: Option<mpsc::Sender<Message>>,
    controller_rx: Option<mpsc::Receiver<Message>>,
}

impl GameController {
    pub fn new() -> Self {
        GameController {
            state: GameState::NoConnection,
            board: OthelloBoard::new(),
            chat_messages: Vec::new(),
            controller_tx: None,
            controller_rx: None
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn get_piece_at(&self, rank: usize, file: usize) -> Option<u8>{
        self.board.get_piece_at(rank, file)
    }

    pub fn get_chat_messages(&self) -> &Vec<String> {
        &self.chat_messages
    }

    pub fn push_chat_message(&mut self, msg: String) {
        self.chat_messages.push(msg.clone());
        self.controller_tx.as_mut().unwrap().send(Message::TextMessage(msg)).unwrap();
    }

    pub fn set_piece_on_board(&mut self, rank: usize, file: usize, which_player: u8)
        -> Result<(), &'static str> {
        self.board.set_piece(rank, file, which_player)?;
        self.controller_tx.as_mut().unwrap().send(Message::SetPiece((rank, file))).unwrap();

        Ok(())
    }

    pub fn check_for_new_message(&mut self) {
        let rx = match self.controller_rx.as_mut() {
            Some(rx) => rx,
            None => return
        };

        if let Some(msg) = rx.try_recv().ok() {
            match msg {
                Message::TextMessage(text) => self.chat_messages.push(text),
                _ => panic!("Can't handle message type")
            }
        }
    }

    pub fn listen_and_connect(&mut self, addr: &str) -> Result<(), Error> {
        let (connection_tx, controller_rx) = mpsc::channel();
        let (controller_tx, connection_rx) = mpsc::channel();
        let mut connection = PeerToPeerConnection::listen_to(addr, 1.0)?;

        self.controller_rx = Some(controller_rx);
        self.controller_tx = Some(controller_tx);

        thread::spawn(move || {
            if let Some(rcv_msg) = connection.wait_for_message() {
                connection_tx.send(rcv_msg).unwrap();
            }

            if let Some(send_msg) = connection_rx.try_recv().ok() {
                connection.send_message(send_msg).unwrap();
            }
        });

        self.state = GameState::Playing;
        Ok(())
    }

    pub fn connect(&mut self, addr: &str) -> Result<(), Error> {
        let (connection_tx, controller_rx) = mpsc::channel();
        let (controller_tx, connection_rx) = mpsc::channel();
        let mut connection = PeerToPeerConnection::connect_to(addr, 1.0)?;

        self.controller_rx = Some(controller_rx);
        self.controller_tx = Some(controller_tx);

        thread::spawn(move || {
            if let Some(rcv_msg) = connection.wait_for_message() {
                connection_tx.send(rcv_msg).unwrap();
            }

            if let Some(send_msg) = connection_rx.try_recv().ok() {
                connection.send_message(send_msg).unwrap();
            }
        });

        self.state = GameState::Playing;
        Ok(())
    }
}
