use std::sync::{Mutex, Arc};

use othello_rs::game_controller::GameController;
use othello_rs::gui::gui_runner::build_game_window;
use othello_rs::networking::start_rpc_server;

fn main() -> eframe::Result<()> {
    let controller = Arc::new(Mutex::new(GameController::new()));

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let server_controller = controller.clone();
    
    runtime.spawn(async move {
        start_rpc_server(server_controller).await.unwrap();
    });

    build_game_window(controller)?;
    Ok(())
}
