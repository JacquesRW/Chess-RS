//* Engine evaluation functions. */

use crate::model::defs::*;
use crate::model::pieces::*;
use crate::model::engine::constants::*;

pub const TOTALPHASE: i64 = 24;

#[inline(always)]
pub fn value(pc: Piece) -> i64 {
    // piece values
    // maybe enum instead?
    match name(pc) { 
        EMPTY => 0,
        PAWN => 100,
        BISHOP => 330,
        KNIGHT => 320,
        ROOK => 500,
        QUEEN => 900,
        KING => 0,
        _ => panic!("Not a valid piece!")
    }
}

#[inline(always)]
pub fn phase_value(pc: Piece) -> i64 {
    // piece values
    // maybe enum instead?
    match name(pc) { 
        EMPTY => 0,
        PAWN => 0,
        BISHOP => 1,
        KNIGHT => 1,
        ROOK => 2,
        QUEEN => 4,
        KING => 0,
        _ => panic!("Not a valid piece!")
    }
}

#[inline(always)]
fn sign(piece: Piece) -> i64 {
    // sign of eval for each colour
    match colour(piece) {
        WHITE => 1, 
        BLACK => -1, 
        EMPTY => 0, 
        _=> panic!("Not valid piece!")}
}

#[inline(always)]
fn mg_weight(piece: Piece, i: usize, j: usize) -> i64 {
    // gives the score for each piece in each position
    (match colour(piece) {
        BLACK => match name(piece) {
            PAWN => PW[i][j],
            KNIGHT => NW[i][j],
            BISHOP => BW[i][j],
            ROOK => RW[i][j],
            QUEEN => QW[i][j],
            KING => KW[i][j],
            _ => panic!("Not valid!")
        },
        WHITE => match name(piece) {
            PAWN => PW[7-i][j],
            KNIGHT => NW[7-i][j],
            BISHOP => BW[7-i][j],
            ROOK => RW[7-i][j],
            QUEEN => QW[7-i][j],
            KING => KW[7-i][j],
            _ => panic!("Not valid!")
        },
        EMPTY => 0,
        _ => panic!("Not valid!")
        }) as i64
}

fn eg_weight(piece: Piece, i: usize, j: usize) -> i64 {
    // gives the score for each piece in each position
    (match colour(piece) {
        BLACK => match name(piece) {
            PAWN => PW2[i][j],
            KNIGHT => NW[i][j],
            BISHOP => BW[i][j],
            ROOK => RW[i][j],
            QUEEN => QW[i][j],
            KING => KW2[i][j],
            _ => panic!("Not valid!")
        },
        WHITE => match name(piece) {
            PAWN => PW2[7-i][j],
            KNIGHT => NW[7-i][j],
            BISHOP => BW[7-i][j],
            ROOK => RW[7-i][j],
            QUEEN => QW[7-i][j],
            KING => KW2[7-i][j],
            _ => panic!("Not valid!")
        },
        EMPTY => 0,
        _ => panic!("Not valid!")
        }) as i64
}

impl Board {
    #[inline(always)]
    fn opening(&self) -> i64 {
        // PLANNED transition to endgame values
        let mut eval: i64 = 0;
        let mut piece: Piece;
        // loops through every square in board
        for i in 0..8 {
            for j in 0..8 {
                piece = self.board[i][j];
                // takes the value of the piece, and the value of its position on the board
                // positive for white, negative for black
                eval += sign(piece) * (value(piece) + mg_weight(piece,i,j));
                }
        }
        // positive for white, negative for black
        eval
    }

    fn endgame(&self) -> i64 {
        // PLANNED transition to endgame values
        let mut eval: i64 = 0;
        let mut piece: Piece;
        // loops through every square in board
        for i in 0..8 {
            for j in 0..8 {
                piece = self.board[i][j];
                // takes the value of the piece, and the value of its position on the board
                // positive for white, negative for black
                eval += sign(piece) * (value(piece) + eg_weight(piece,i,j));
                }
        }
        // positive for white, negative for black
        eval
    }

    pub fn phase(&self) -> i64 { ((TOTALPHASE - self.phase) * 256 + (TOTALPHASE / 2)) / TOTALPHASE }

    pub fn evaluate(&self) -> i64 {
        let phase = self.phase();
        sign(self.color) * (((256 - phase) * self.opening()) + (phase * self.endgame())) / 256
    }
}