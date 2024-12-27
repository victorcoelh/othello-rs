use crate::game_logic::OthelloBoard;
use crate::networking::RpcClient;
use std::sync::{Arc, Mutex};

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
    pub state: GameState,
    pub board: OthelloBoard,
    pub is_host: bool,
    pub player_turn: bool,
    pub opponent_passed: bool,
    pub error_queue: Arc<Mutex<Vec<String>>>,
    chat_messages: Vec<String>,
    rpc_client: Option<RpcClient>,
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
            rpc_client: None,
            error_queue: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn get_piece_at(&self, rank: usize, file: usize) -> Option<u8>{
        self.board.get_piece_at(rank, file)
    }

    pub fn get_chat_messages(&self) -> &Vec<String> {
        &self.chat_messages
    }

    pub fn connect_to(&mut self, ip_addr: &str, host: bool) {
        let mut client = RpcClient::new(ip_addr, self.error_queue.clone()).unwrap();
        client.connect_to();
        self.rpc_client = Some(client);

        self.state = GameState::Playing;
        self.is_host = host;
        self.player_turn = host;
    }

    pub fn try_set_piece_on_board(&mut self, rank: usize, file: usize, from_opponent: bool) {
        if !from_opponent {
            if !self.player_turn {
                self.chat_messages.push("ERROR: Wait for your opponent's turn!".to_string());
                return ()
            }
            self.rpc_client.as_mut().unwrap().set_piece(rank, file);
        }

        let from_opponent = self.swap_player_if_not_host(from_opponent);
        if let Err(error) = self.board.set_piece(rank, file, from_opponent as u8) {
            self.chat_messages.push(format!("ERROR: {}", error));
            return ()
        }

        let (p1_pieces, p2_pieces) = self.board.count_pieces();
        if p1_pieces == 0 || p2_pieces == 0 {
            self.opponent_passed = true;
            self.try_pass_turn();
        }

        self.player_turn = !self.player_turn;
        self.opponent_passed = false;
    }

    pub fn try_pass_turn(&mut self) {
        if !self.player_turn {
            self.chat_messages.push("ERROR: Can't pass if it is not your turn!".to_string());
            return ();
        }

        if self.opponent_passed {
            let player_won = self.check_if_player_won();
            self.state = GameState::GameEnded(player_won);
            self.rpc_client.as_mut().unwrap().end_game(false); //ajeitar dps
        }

        self.player_turn = false;
        self.opponent_passed = false;
        self.rpc_client.as_mut().unwrap().change_turn();
    }

    pub fn push_chat_message(&mut self, msg: String, from_opponent: bool) {
        let msg_with_prefix = match from_opponent {
            false => {
                self.rpc_client.as_mut().unwrap().send_chat_message(msg.clone());
                format!("player: {}", msg)
            },
            true => format!("opponent: {}", msg)
        };
        self.chat_messages.push(msg_with_prefix);
    }

    pub fn push_warning_to_chat(&mut self, msg: &str) {
        self.chat_messages.push(format!("WARNING: {}", msg))
    }

    pub fn surrender(&mut self) {
        self.rpc_client.as_mut().unwrap().end_game(true);
        self.state = GameState::GameEnded(GameResult::PlayerLost);
    }

    pub fn undo_last_move(&mut self) {
        self.board.revert_to_last_state();
        self.player_turn = !self.player_turn;
        self.opponent_passed = false;

        self.rpc_client.as_mut().unwrap().undo_move();
    }

    pub fn restart_game(&mut self) {
        self.state = GameState::NoConnection;
        self.board = OthelloBoard::new();
        self.chat_messages = Vec::new();
    }

    fn swap_player_if_not_host(&self, from_opponent: bool) -> bool {
        if !self.is_host {
            !from_opponent
        } else {
            from_opponent
        }
    }

    pub fn check_if_player_won(&self) -> GameResult {
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
