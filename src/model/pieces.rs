//* Stuff for the Pieces type. */
use crate::model::defs::Piece;

#[inline(always)]
pub fn colour(pc: Piece) -> Piece {
    (pc >> 3) << 3
}
#[inline(always)]
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

#[inline(always)]
pub fn other_colour(color: Piece) -> Piece {
    match color {
        WHITE => BLACK,
        BLACK => WHITE,
        EMPTY => EMPTY,
        _ => panic!("Not valid colour.")
    }
}

#[inline(always)]
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

#[inline(always)]
pub fn repr(pc: Piece) -> &'static str {
    match colour(pc) {
        BLACK => match name(pc) {
            PAWN => " ♙",
            BISHOP => " ♗",
            KNIGHT => " ♘",
            ROOK => " ♖",
            QUEEN => " ♕",
            KING => " ♔",
            _ => panic!("Not a valid piece!")
        },
        WHITE => match name(pc) {
            PAWN => " ♟",
            BISHOP => " ♝",
            KNIGHT => " ♞",
            ROOK => " ♜",
            QUEEN => " ♛",
            KING => " ♚",
            _ => panic!("Not a valid piece!")
        },
        EMPTY => "  ",
        _ => panic!("Not valid colour.")
    }
}

#[inline(always)]
pub fn value(pc: Piece) -> i64 {
    let x = match colour(pc) {
        WHITE => 1,
        BLACK => -1,
        EMPTY => 0,
        _ => panic!("Not valid colour!")
    };
    let y =match name(pc) { 
        EMPTY => 0,
        PAWN => 10,
        BISHOP => 30,
        KNIGHT => 30,
        ROOK => 50,
        QUEEN => 90,
        KING => 900,
        _ => panic!("Not a valid piece!")
    };
    x * y
}