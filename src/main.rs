use othello_rs::game_controller::GameController;
use othello_rs::gui::gui_runner::build_game_window;

fn main() -> eframe::Result<()> {
    let controller = GameController::new();
    build_game_window(controller)?;

    Ok(())
}
