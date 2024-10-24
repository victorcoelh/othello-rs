use othello_rs::game_controller::GameController;
use othello_rs::networking::Message;
use othello_rs::gui::game_window::build_game_window;

fn main() -> eframe::Result<()> {
    let message = Message::TextMessage("I love cock".to_string());
    let decoded = Message::from_bytes(&message.to_bytes());

    match decoded.unwrap() {
        Message::TextMessage(text) => println!("{text}"),
        _ => panic!("nah")
    }

    let controller = GameController::new();
    build_game_window(controller)?;

    Ok(())
}
