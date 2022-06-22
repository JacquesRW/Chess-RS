//* STRUCT AND TYPE ALIAS DECLARATIONS */

#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    // piece = 'e','P','K','Q','R','N' or 'B'
    pub piece: char,
    // color = 0,1,2
    pub color: usize
}

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

pub struct Board {
    // current board
    pub board: Array,
    // last move played - for castling and en passant
    pub last_move: Move,
    // tracking if can castle
    // castle = [[has white castles, had black castled],[White QS, White KS],[Black QS, Black KS]]
    pub castle: [[bool;2];3],
    pub color: usize
}