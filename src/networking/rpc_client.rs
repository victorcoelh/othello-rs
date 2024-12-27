use std::sync::{Arc, Mutex};

use tonic::transport::Channel;

use crate::othello_rpc::chat_client::ChatClient;
use crate::othello_rpc::board_client::BoardClient;
use crate::othello_rpc::game_flow_client::GameFlowClient;
use crate::othello_rpc::{ChatRequest, Empty, EndRequest, PieceRequest};
use crate::RpcResult;

pub struct RpcClient{
    chat_client: ChatClient<Channel>,
    board_client: BoardClient<Channel>,
    game_flow_client: GameFlowClient<Channel>,
    error_queue: Arc<Mutex<Vec<String>>>,
}

impl RpcClient {
    pub fn new(ip_addr: &str, error_queue: Arc<Mutex<Vec<String>>>) -> Result<Self, tonic::transport::Error> {
        let chat_url = format!("http://{}:11069", ip_addr);
        let board_url = format!("http://{}:11069", ip_addr);
        let game_url = format!("http://{}:11069", ip_addr);

        let initializer = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        initializer.block_on(async move {
            Ok(RpcClient {
                chat_client: ChatClient::connect(chat_url).await?,
                board_client: BoardClient::connect(board_url).await?,
                game_flow_client: GameFlowClient::connect(game_url).await?,
                error_queue: error_queue,
            })
        })
    }

    pub fn send_chat_message(&mut self, msg: String) {
        let request = ChatRequest { msg: msg };
        let mut client = self.chat_client.clone();
        let error_queue = self.error_queue.clone();

        tokio::spawn(async move {
            let result = client.send_message(request).await;
            handle_error(result, error_queue);
        });
    }

    pub fn set_piece(&mut self, rank: usize, file: usize) {
        let request = PieceRequest { rank: rank as i32, file: file as i32 };
        let mut client = self.board_client.clone();
        let error_queue = self.error_queue.clone();

        tokio::spawn(async move {
            let result = client.set_piece(request).await;
            handle_error(result, error_queue);
        });
    }

    pub fn end_game(&mut self, opponent_won: bool) {
        let request = EndRequest { game_won: opponent_won };
        let mut client = self.game_flow_client.clone();
        let error_queue = self.error_queue.clone();

        tokio::spawn(async move {
            let result = client.end_game(request).await;
            handle_error(result, error_queue);
        });
    }

    pub fn change_turn(&mut self) {
        let request = Empty { };
        let mut client = self.game_flow_client.clone();
        let error_queue = self.error_queue.clone();

        tokio::spawn(async move {
            let result = client.change_turn(request).await;
            handle_error(result, error_queue);
        });
    }

    pub fn undo_move(&mut self) {
        let request = Empty { };
        let mut client = self.game_flow_client.clone();
        let error_queue = self.error_queue.clone();

        tokio::spawn(async move {
            let result = client.undo_move(request).await;
            handle_error(result, error_queue);
        });
    }
}

fn handle_error(result: RpcResult, error_queue: Arc<Mutex<Vec<String>>>) {
    if let Err(status) = result {
        error_queue.lock().expect("Cannot obtain Mutex resource.").push(
            format!("Error {} while sending message: {}", status.code(), status.message())
        );
    }
}
