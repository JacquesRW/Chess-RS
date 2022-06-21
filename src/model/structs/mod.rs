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
    pub target: Piece,
    pub orig: Square,
    pub dest: Square 
}

pub struct Board {
    pub board: Array,
    pub last_move: Move,
    pub castle: [[bool;2];3],
    pub color: usize
}