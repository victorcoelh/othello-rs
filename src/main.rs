#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use othello_rs::game_logic::OthelloBoard;
use othello_rs::Color;
use othello_rs::gui::build_game_window;

fn main() -> eframe::Result<()> {
    let player_colors: (Color, Color) = ((255, 255, 255), (0, 0, 0));
    let mut board = OthelloBoard::new(player_colors);

    board.set_piece('b', 4, 0).unwrap();
    board.print_board();

    build_game_window()
}
