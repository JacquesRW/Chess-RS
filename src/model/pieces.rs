//* Stuff for the Pieces type. */
use crate::model::defs::Piece;

#[inline(always)]
pub fn colour(pc: Piece) -> Piece {
    // get colour of piece
    (pc >> 3) << 3
}

#[inline(always)]
pub fn name(pc: Piece) -> Piece {
    // gets type of piece
    (pc << 5) >> 5
}

// colours and empty
pub const EMPTY: Piece = 0b00000000;
pub const WHITE: Piece = 0b00001000;
pub const BLACK: Piece = 0b00010000;

// piece types
pub const PAWN: Piece = 0b00000001;
pub const KNIGHT: Piece = 0b00000010;
pub const BISHOP: Piece = 0b00000011;
pub const ROOK: Piece = 0b00000100;
pub const QUEEN: Piece = 0b00000101;
pub const KING: Piece = 0b00000110;

#[inline(always)]
pub fn other_colour(color: Piece) -> Piece {
    // returns opposite colour
    match color {
        WHITE => BLACK,
        BLACK => WHITE,
        EMPTY => EMPTY,
        _ => panic!("Not valid colour.")
    }
}

#[inline(always)]
pub fn as_string(pc: Piece) -> String {
    // colour of the piece
    let color = match colour(pc) {
        WHITE => "white",
        BLACK => "black",
        EMPTY => "empty",
        _ => panic!("Not valid colour.")
    };
    // type of piece
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
    // piece string for console output
    match colour(pc) {
        BLACK => match name(pc) {
            PAWN => "♙",
            BISHOP => "♗",
            KNIGHT => "♘",
            ROOK => "♖",
            QUEEN => "♕",
            KING => "♔",
            _ => panic!("Not a valid piece!")
        },
        WHITE => match name(pc) {
            PAWN => "♟",
            BISHOP => "♝",
            KNIGHT => "♞",
            ROOK => "♜",
            QUEEN => "♛",
            KING => "♚",
            _ => panic!("Not a valid piece!")
        },
        EMPTY => " ",
        _ => panic!("Not valid colour.")
    }
}
