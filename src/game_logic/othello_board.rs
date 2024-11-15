use crate::Position;

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

impl PartialEq for OthelloPiece {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

pub struct OthelloBoard{
    board_state: [[Option<OthelloPiece>; 8]; 8]
}

impl OthelloBoard{
    pub fn new() -> Self {
        let mut board = [[None; 8]; 8];
        board[3][3] = Some(OthelloPiece::new(1));
        board[3][4] = Some(OthelloPiece::new(0));
        board[4][3] = Some(OthelloPiece::new(0));
        board[4][4] = Some(OthelloPiece::new(1));

        OthelloBoard { board_state: board }
    }

    pub fn get_piece_at(&self, rank: usize, file: usize) -> Option<u8>{
        self.board_state[file][rank].map(|piece| piece.state)
    }

    pub fn set_piece(&mut self, rank: usize, file: usize, which_player: u8) -> Result<(), &'static str> {
        if rank > 7 || file > 7 {
            return Err("Invalid piece position given. Either the rank or the file are outside \
            of the game board's upper bounds. Maximum rank: h, maximum file: 8.");
        }

        if let Some(_) = self.board_state[file][rank] {
            return Err("There already is a piece at the given position. Pieces must be placed on \
            empty squares.")
        }
        
        let new_piece = OthelloPiece::new(which_player);
        self.board_state[file][rank] = Some(new_piece);
        self.flip_pieces_if_needed(rank, file);

        Ok(())
    }

    pub fn count_pieces(&self) -> (usize, usize) {
        let mut p1_pieces: usize = 0;
        let mut p2_pieces: usize = 0;

        let _: Vec<_> = self.board_state.as_flattened().into_iter().map(|piece| {
            if let Some(piece) = piece {
                match piece.state {
                    0 => p1_pieces += 1,
                    1 => p2_pieces += 1,
                    _ => panic!("Unexpected value received for an Othello Piece.")
                };
            }
        }).collect();

        (p1_pieces, p2_pieces)
    }

    pub fn print_board(&self) {
        for rank in self.board_state {
            println!("{rank:?}")
        } 
    }

    fn flip_pieces_if_needed(&mut self, rank: usize, file: usize) {
        let should_flip = self.check_for_flanks(rank, file);
        let current_state = self.board_state[file][rank].unwrap().state;

        for (rank, file) in should_flip {
            self.board_state[file][rank] = Some(OthelloPiece { state: current_state })
        }
    }

    fn check_for_flanks(&mut self, rank: usize, file: usize) -> Vec<Position> {
        let mut should_flip: Vec<Position> = Vec::new();
        let current_piece = self.board_state[file][rank];

        for direction in self.cast_rays(rank, file) {
            for (i, (rank, file)) in direction.iter().enumerate() {
                if current_piece == self.board_state[*file][*rank] {
                    println!("{:?}", &direction[..i]);
                    should_flip.extend_from_slice(&direction[..i]);
                }
            }
        }
        should_flip
    }

    fn cast_rays(&self, rank: usize, file: usize) -> Vec<Vec<Position>> {
        let mut hit_rays: Vec<Vec<Position>> = Vec::new();

        hit_rays.push((rank+1..8).map(|x| (x, file)).collect());
        hit_rays.push((0..rank).rev().map(|x| (x, file)).collect());
        hit_rays.push((file+1..8).map(|y| (rank, y)).collect());
        hit_rays.push((0..file).rev().map(|y| (rank, y)).collect());

        println!("{:?}", hit_rays);
        hit_rays
    }
}
