use crate::util::piece::Piece;

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub target: Piece,
    pub orig: [usize;2],
    pub dest: [usize;2] 
}

impl Move {

    pub fn null() -> Move {
        Move {target: Piece::empty(), orig: [0,0], dest: [0,0]}
    }
    pub fn new(piece: Piece, origin: [usize;2], destination: [usize;2]) -> Move {
        Move {target: piece, orig: origin, dest: destination}
    }

    pub fn log(&self) {
        println!("Moving piece {} from {:?} to {:?}", self.target.string_form(), self.orig, self.dest)
    }

}