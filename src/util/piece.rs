#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    // piece = 'e','P','K','Q','R','N' or 'B'
    pub piece: char,
    // color = 0,1,2
    pub color: usize
}
impl Piece {
    pub fn string_form(&self) -> String {
        format!("{}{}", &self.piece.to_string(), &self.color.to_string())
    }
    pub fn empty() -> Piece {
        Piece {piece: 'e', color: 0}
    }
    pub fn king(colour: usize) -> Piece {
        Piece {piece: 'K', color: colour}
    }
    pub fn pawn(colour: usize) -> Piece {
        Piece {piece: 'P', color: colour}
    }
    pub fn knight(colour: usize) -> Piece {
        Piece {piece: 'N', color: colour}
    }
    pub fn bishop(colour: usize) -> Piece {
        Piece {piece: 'B', color: colour}
    }
    pub fn rook(colour: usize) -> Piece {
        Piece {piece: 'R', color: colour}
    }
    pub fn queen(colour: usize) -> Piece {
        Piece {piece: 'Q', color: colour}
    }
}