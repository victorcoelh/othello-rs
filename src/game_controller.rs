use crate::game_logic::OthelloBoard;
use crate::networking::{Message, PeerToPeerConnection};
use std::io::Error;

pub enum GameState {
    NoConnection,
    Playing,
    GameEnded,
}

pub struct GameController {
    state: GameState,
    board: OthelloBoard,
    peer: Option<PeerToPeerConnection>,
    chat_messages: Vec<String>,
}

impl GameController {
    pub fn new() -> Self {
        GameController {
            state: GameState::NoConnection,
            board: OthelloBoard::new(),
            peer: None,
            chat_messages: Vec::new()
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.chat_messages
    }

    pub fn set_piece_on_board(&mut self, rank: char, file: u8, which_player: u8)
        -> Result<(), &'static str> {
        self.board.set_piece(rank, file, which_player)
    }

    pub fn send_message(&mut self, message: Message) -> Result<(), Error> {
        self.peer.as_mut().unwrap().send_message(message)
    }

    pub fn handle_new_messages(&mut self) -> Result<(), Error> {
        match self.peer.as_mut().unwrap().wait_for_message()? {
            Message::TextMessage(text) => self.chat_messages.push(text),
            _ => panic!("Can't handle message type")
        }
        Ok(())
    }

    pub fn listen_and_connect(&mut self, addr: &str) -> Result<(), Error> {
        println!("Waiting for connection to socket: {addr}");
        self.peer = Some(PeerToPeerConnection::listen_to(addr)?);
        self.state = GameState::Playing;
        Ok(())
    }

    pub fn connect(&mut self, addr: &str) -> Result<(), Error> {
        println!("Connecting to socket: {addr}");
        self.peer = Some(PeerToPeerConnection::connect_to(addr)?);
        self.state = GameState::Playing;
        Ok(())
    }
}
