//* STRUCT AND TYPE ALIAS DECLARATIONS */

pub type Piece = u8;
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
    pub last_move: Move, // PLANNED REFACTOR to "en passant square" FOR PERFORMANCE
    pub castle: [[bool;2];2], // PLANNED REFACTOR TO u8 FOR CONVENIENCE
    // current player
    pub color: u8,
    // king locations
    pub kings: [Square; 2],
    // game phase heuristic
    pub phase: i64
}