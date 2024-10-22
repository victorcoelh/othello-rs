use crate::Color;

#[derive(Copy, Clone, Debug)]
struct OthelloPiece{
    state: u8
}

impl OthelloPiece{
    fn new(which_player: u8) -> Self {
        if which_player != 0 && which_player != 1 {
            panic!("Received an invalid state for an Othello Piece. A piece must always be
            in a binary state, representing a piece for either player.");
        }
        OthelloPiece { state: which_player }
    }
}

pub struct OthelloBoard{
    piece_colors: (Color, Color),
    board_state: [[Option<OthelloPiece>; 8]; 8]
}

impl OthelloBoard{
    pub fn new(piece_colors: (Color, Color)) -> Self {
        let empty_board = [[None; 8]; 8];
        OthelloBoard {
            piece_colors: piece_colors,
            board_state: empty_board
        }
    }

    pub fn set_piece(&mut self, rank: char, file: u8, which_player: u8) -> Result<(), &'static str> {
        let rank = ((rank as u32) - ('a' as u32)) as usize;
        let file = (file - 1) as usize;

        if rank > 7 || file > 7 {
            return Err("Invalid piece position given. Either the rank or the file are outside \
            of the game board's upper bounds. Maximum rank: h, maximum file: 8.");
        }

        if let Some(_) = self.board_state[rank][file] {
            return Err("There already is a piece at the given position. Pieces must be placed on \
            empty squares.")
        }
        
        let new_piece = OthelloPiece::new(which_player);
        self.board_state[file][rank] = Some(new_piece);

        Ok(())
    }

    pub fn print_board(&self) {
        for rank in self.board_state {
            println!("{rank:?}")
        } 
    }
}
