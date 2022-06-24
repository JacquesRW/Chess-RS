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

#[derive(Clone)]
pub struct Board {
    // current board
    pub board: Array,
    // last move played - for castling and en passant
    pub last_move: Move,
    // tracking if can castle
    // castle = [[has white castles, had black castled],[White QS, White KS],[Black QS, Black KS]]
    pub castle: [[bool;2];3],
    // current player
    pub color: u8,
    // capture on last move, if any
    pub capture: Option<Piece>,
    pub prev_castle: [[bool;2];3],
    pub prev_move: Move,
    pub best_move: Move
}