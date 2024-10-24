use crate::game_logic::OthelloBoard;
use crate::networking::PeerToPeerConnection;
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

    pub fn get_state(&self) -> &GameState {
        &self.state
    }
}

