//* Implementation of methods for the Piece struct. */
use crate::model::defs::Piece;

impl Piece {
    pub fn repr(&self) -> String {
        format!("{}{}", self.piece.to_string(),self.color.to_string())
    }
    pub fn to_string(&self) -> String {
        let x = match &self.piece {
            'P' => "pawn",
            'K' => "king",
            'Q' => "queen",
            'R' => "rook",
            'B' => "bishop",
            'N' => "knight",
            'e' => "Empty",
            _ => panic!("Not a valid piece")
        };
        let y = match &self.color {
            0 => "",
            1 => "white",
            2 => "black",
            _ => panic!("Not a valid colour.")
        };
        format!("{y} {x}")
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
