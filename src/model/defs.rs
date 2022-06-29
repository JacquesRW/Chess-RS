//* STRUCT AND TYPE ALIAS DECLARATIONS */

pub type Piece = u8;
pub type Castle = u8;
pub type Array = [[Piece;8];8];
pub type Square = [usize;2];

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    // piece to move
    pub target: Piece,
    // original square of piece to move
    pub orig: Square,
    // destination square of move
    pub dest: Square 
}

#[derive(Clone, Copy)]
pub struct ScoredMove {
    pub m: Move,
    pub s: i64
}

#[derive(Clone)]
pub struct Board {
    // current board
    pub board: Array,
    // PLANNED REFACTOR to "en passant square" to comply with standard practise
    pub last_move: Move, 
    // castling rights
    pub castle: Castle,
    // current player
    pub color: u8,
    // king locations
    pub kings: [Square; 2],
    // game phase heuristic
    pub phase: i64
}

// castling stuff - not sure where else it should go
pub const WHITE_QS: Castle = 0b00001000; 
pub const WHITE_KS: Castle = 0b00000100; 
pub const BLACK_QS: Castle = 0b00000010; 
pub const BLACK_KS: Castle = 0b00000001; 
pub const NO_RIGHTS: Castle = 0b00000000;