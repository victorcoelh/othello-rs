use crate::game_logic::OthelloBoard;
use crate::networking::{Message, PeerToPeerConnection};
use std::io::Error;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
use std::thread;

#[derive(Copy, Clone)]
pub enum GameResult {
    PlayerWon,
    PlayerLost,
    Tie
}

pub enum GameState {
    NoConnection,
    Playing,
    GameEnded(GameResult),
}

pub struct GameController {
    state: GameState,
    board: OthelloBoard,
    is_host: bool,
    player_turn: bool,
    opponent_passed: bool,
    chat_messages: Vec<String>,
    controller_tx: Option<mpsc::Sender<Message>>,
    controller_rx: Option<mpsc::Receiver<Message>>,
}

impl GameController {
    pub fn new() -> Self {
        GameController {
            state: GameState::NoConnection,
            board: OthelloBoard::new(),
            is_host: false,
            player_turn: true,
            opponent_passed: false,
            chat_messages: Vec::new(),
            controller_tx: None,
            controller_rx: None,
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
    
    pub fn get_player_turn(&self) -> bool {
        self.player_turn
    }

    pub fn get_is_host(&self) -> bool {
        self.is_host
    }

    pub fn try_set_piece_on_board(&mut self, rank: usize, file: usize, which_player: bool) {
        if !which_player {
            if !self.player_turn {
                self.chat_messages.push("ERROR: Wait for your opponent's turn!".to_string());
                return ()
            }
            self.send_message_to_connection(Message::SetPiece((rank, file)));
        }

        let which_player = self.swap_player_if_not_host(which_player);
        if let Err(error) = self.board.set_piece(rank, file, which_player as u8) {
            self.chat_messages.push(format!("ERROR: {}", error));
        }

        self.player_turn = !self.player_turn;
    }

    pub fn try_pass_turn(&mut self) {
        if !self.player_turn {
            self.chat_messages.push("ERROR: Can't pass if it is not your turn!".to_string());
            return ();
        }

        if self.opponent_passed {
            let player_won = self.check_if_player_won();
            self.state = GameState::GameEnded(player_won);
            self.send_message_to_connection(Message::GameEnded());
        }

        self.player_turn = false;
        self.send_message_to_connection(Message::PassTurn());
    }

    pub fn push_chat_message(&mut self, msg: String, which_player: bool) {
        let msg_with_prefix = match which_player {
            false => {
                self.send_message_to_connection(Message::TextMessage(msg.clone()));
                format!("player: {}", msg)
            },
            true => format!("opponent: {}", msg)
        };
        self.chat_messages.push(msg_with_prefix);
    }

    pub fn surrender(&mut self) {
        self.send_message_to_connection(Message::Surrender());
        self.state = GameState::GameEnded(GameResult::PlayerLost);
    }

    pub fn undo_last_move(&mut self) {
        self.board.revert_to_last_state();
        self.player_turn = !self.player_turn;

        self.send_message_to_connection(Message::UndoMove());
    }

    pub fn check_for_new_message(&mut self) {
        let rx = match self.controller_rx.as_mut() {
            Some(rx) => rx,
            None => return
        };

        if let Some(msg) = rx.try_recv().ok() {
            self.opponent_passed = false;

            match msg {
                Message::TextMessage(text) => self.push_chat_message(text, true),
                Message::Surrender() => self.state = GameState::GameEnded(GameResult::PlayerWon),
                Message::TestConnection() => (),
                Message::SetPiece((x, y)) => {
                    self.try_set_piece_on_board(x, y, true);
                },
                Message::UndoMove() => {
                    self.board.revert_to_last_state();
                    self.player_turn = !self.player_turn;
                    self.chat_messages.push("WARNING: The last move was undone by the opponent.".to_string());
                },
                Message::GameEnded() => {
                    let player_won = self.check_if_player_won();
                    self.state = GameState::GameEnded(player_won);
                },
                Message::PassTurn() => {
                    self.player_turn = !self.player_turn;
                    self.opponent_passed = true;
                    self.chat_messages.push("WARNING: Opponent forfeited their turn.".to_string());
                },
            }
        }
    }

    pub fn restart_game(&mut self) {
        self.state = GameState::NoConnection;
        self.board = OthelloBoard::new();
        self.chat_messages = Vec::new();
        self.controller_rx = None;
        self.controller_tx = None;
    }

    pub fn listen_and_connect(&mut self, addr: &str, error_tx: mpsc::Sender<String>) -> Result<(), Error> {
        let (connection_tx, controller_rx) = mpsc::channel();
        let (controller_tx, connection_rx) = mpsc::channel();
        let mut connection =
            PeerToPeerConnection::listen_to(addr, 1.0, error_tx)?;

        self.controller_rx = Some(controller_rx);
        self.controller_tx = Some(controller_tx);

        thread::spawn(move || {
            let mut start = SystemTime::now();
            let time_limit = Duration::from_secs(5);

            loop {
                if start.elapsed().unwrap_or(Duration::ZERO) > time_limit {
                    connection.test_connection().unwrap();
                    start = SystemTime::now();
                }

                if let Some(rcv_msg) = connection.wait_for_message() {
                    connection_tx.send(rcv_msg).unwrap(); // can safely unwrap
                }
    
                if let Some(send_msg) = connection_rx.try_recv().ok() {
                    connection.send_message(send_msg).unwrap();
                }
            }
        });

        self.state = GameState::Playing;
        self.player_turn = true;
        self.is_host = true;
        Ok(())
    }

    pub fn connect(&mut self, addr: &str, error_tx: mpsc::Sender<String>) -> Result<(), Error> {
        let (connection_tx, controller_rx) = mpsc::channel();
        let (controller_tx, connection_rx) = mpsc::channel();
        let mut connection =
            PeerToPeerConnection::connect_to(addr, 1.0, error_tx)?;

        self.controller_rx = Some(controller_rx);
        self.controller_tx = Some(controller_tx);

        thread::spawn(move || {
            let mut start = SystemTime::now();
            let time_limit = Duration::from_secs_f32(2.5);

            loop {
                if start.elapsed().unwrap_or(Duration::ZERO) > time_limit {
                    connection.test_connection().unwrap();
                    start = SystemTime::now();
                }

                if let Some(rcv_msg) = connection.wait_for_message() {
                    connection_tx.send(rcv_msg).unwrap(); // can safely unwrap
                }
    
                if let Some(send_msg) = connection_rx.try_recv().ok() {
                    connection.send_message(send_msg).unwrap();
                }
            }
        });

        self.state = GameState::Playing;
        self.player_turn = false;
        Ok(())
    }

    fn send_message_to_connection(&mut self, message: Message) {
        self.controller_tx.as_mut().unwrap().send(message).unwrap();
    }

    fn swap_player_if_not_host(&self, which_player: bool) -> bool {
        if !self.is_host {
            !which_player
        } else {
            which_player
        }
    }

    fn check_if_player_won(&self) -> GameResult {
        let (p1_pieces, p2_pieces) = self.board.count_pieces();

        if p1_pieces == p2_pieces {
            return GameResult::Tie
        }

        if self.is_host && p1_pieces > p2_pieces {
            return GameResult::PlayerWon
        }

        if !self.is_host && p2_pieces > p1_pieces {
            return GameResult::PlayerWon
        }

        GameResult::PlayerLost
    }
}
