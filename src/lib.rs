pub mod game_logic;
pub mod gui;
pub mod networking;
pub mod game_controller;

pub type Color = (u8, u8, u8);
pub type Position = (usize, usize);
pub type RpcResult = Result<tonic::Response<othello_rpc::Empty>, tonic::Status>;

pub mod othello_rpc {
    tonic::include_proto!("othello_rpc");
}
