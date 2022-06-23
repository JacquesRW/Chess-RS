//* Implementation of methods for the Piece struct. */
use crate::model::defs::Piece;

pub fn colour(pc: Piece) -> Piece {
    (pc >> 3) << 3
}
pub fn name(pc: Piece) -> Piece {
    (pc << 5) >> 5
}


pub const EMPTY: Piece = 0b00000000;
pub const WHITE: Piece = 0b00001000;
pub const BLACK: Piece = 0b00010000;

pub const PAWN: Piece = 0b00000001;
pub const KNIGHT: Piece = 0b00000010;
pub const BISHOP: Piece = 0b00000011;
pub const ROOK: Piece = 0b00000100;
pub const QUEEN: Piece = 0b00000101;
pub const KING: Piece = 0b00000110;

pub fn other_colour(color: Piece) -> Piece {
    match color {
        WHITE => BLACK,
        BLACK => WHITE,
        EMPTY => EMPTY,
        _ => panic!("Not valid colour.")
    }
}

pub fn as_string(pc: Piece) -> String {
    let color = match colour(pc) {
        WHITE => "white",
        BLACK => "black",
        EMPTY => "empty",
        _ => panic!("Not valid colour.")
    };
    let piece = match name(pc) {
        PAWN => "pawn",
        BISHOP => "bishop",
        KNIGHT => "knight",
        ROOK => "rook",
        QUEEN => "queen",
        KING => "king",
        EMPTY => "",
        _ => panic!("Not a valid piece!")
    };
    format!("{color} {piece}")
}

pub fn repr(pc: Piece) -> String {
    let color = match colour(pc) {
        WHITE => "1",
        BLACK => "2",
        EMPTY => "0",
        _ => panic!("Not valid colour.")
    };
    let piece = match name(pc) {
        PAWN => "P",
        BISHOP => "B",
        KNIGHT => "N",
        ROOK => "R",
        QUEEN => "Q",
        KING => "K",
        EMPTY => "e",
        _ => panic!("Not a valid piece!")
    };
    format!("{piece}{color}")
}