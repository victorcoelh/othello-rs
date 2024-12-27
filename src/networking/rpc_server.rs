use std::sync::{Arc, Mutex};

use tonic::{Request, Response, transport::Server};

use crate::game_controller::{GameController, GameState};
use crate::RpcResult;
use crate::othello_rpc::chat_server::{ChatServer, Chat};
use crate::othello_rpc::game_flow_server::{GameFlowServer, GameFlow};
use crate::othello_rpc::board_server::{BoardServer, Board};
use crate::othello_rpc::{ChatRequest, Empty, EndRequest, PieceRequest};

#[derive(Clone)]
struct RpcServer {
    controller: Arc<Mutex<GameController>>
}

impl RpcServer {
    fn new(controller: Arc<Mutex<GameController>>) -> Self {
        RpcServer { controller }
    }

    fn build_response(&self) -> Response<Empty> {
        Response::new(Empty { })
    }

    fn lock_controller(&self) -> Result<std::sync::MutexGuard<GameController>, tonic::Status> {
        self.controller.lock().map_err(|_| {
            tonic::Status::new(tonic::Code::Internal, "Error while locking Mutex")
        })
    }
}

#[tonic::async_trait]
impl Chat for RpcServer {
    async fn send_message(&self, request: Request<ChatRequest>) -> RpcResult {
        let text_msg = request.into_inner().msg;

        let mut controller = self.lock_controller()?;
        controller.push_chat_message(text_msg, true);

        Ok(self.build_response())
    }
}

#[tonic::async_trait]
impl Board for RpcServer {
    async fn set_piece(&self, request: Request<PieceRequest>) -> RpcResult {
        let pieces = request.into_inner();
        let (rank, file) = (pieces.rank as usize, pieces.file as usize);

        let mut controller = self.lock_controller()?;
        controller.try_set_piece_on_board(rank, file, true);

        Ok(self.build_response())
    }
}

#[tonic::async_trait]
impl GameFlow for RpcServer {
    async fn end_game(&self, _request: Request<EndRequest>) -> RpcResult {
        let mut controller = self.lock_controller()?;
        let player_won = controller.check_if_player_won();
        controller.state = GameState::GameEnded(player_won);

        Ok(self.build_response())
    }

    async fn change_turn(&self, _request: Request<Empty>) -> RpcResult {
        let mut controller = self.lock_controller()?;
        controller.opponent_passed = true;
        controller.player_turn = !controller.player_turn;
        controller.push_warning_to_chat("Opponent forfeited their turn.");

        Ok(self.build_response())
    }

    async fn undo_move(&self, _request: Request<Empty>) -> RpcResult {
        let mut controller = self.lock_controller()?;

        controller.board.revert_to_last_state();
        controller.player_turn = !controller.player_turn;
        controller.push_warning_to_chat("The last move was undone by the opponent.");

        Ok(self.build_response())
    }

    async fn test_connection(&self, _request: Request<Empty>) -> RpcResult {
        Ok(self.build_response())
    }
}

pub async fn start_rpc_server(game_controller: Arc<Mutex<GameController>>)
    -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("http://127.0.0.1:11069").parse()?;
    let server = RpcServer::new(game_controller);

    Server::builder()
        .add_service(ChatServer::new(server.clone()))
        .add_service(BoardServer::new(server.clone()))
        .add_service(GameFlowServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
