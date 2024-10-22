use othello_rs::game_logic::OthelloBoard;
use othello_rs::Color;

fn main() {
    let player_colors: (Color, Color) = ((255, 255, 255), (0, 0, 0));
    let mut board = OthelloBoard::new(player_colors);

    board.set_piece('b', 4, 0).unwrap();
    board.print_board();
}
