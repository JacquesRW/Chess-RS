//* Engine evaluation functions. */

use crate::model::defs::*;
use std::cmp::max;
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

#[inline(always)]
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
    fn kings_endgame(&self, colour: u8) -> i64 {
        let friendly = self.kings[colour_to_index(colour)];
        let opponent = self.kings[colour_to_index(other_colour(colour))];
        let opp_ctr_dst = max(3-(opponent[0] as i64),(opponent[0] as i64) - 4) + max(3-(opponent[1] as i64), (opponent[1] as i64) - 4);
        let k_dst = ((friendly[0] as i64) - (opponent[0] as i64)).abs() + ((friendly[1] as i64) - (opponent[1] as i64)).abs();
        return 14 + opp_ctr_dst - k_dst
    }

    #[inline(always)]
    pub fn evaluate(&self) -> i64 {
        let mut mat_eval: i64 = 0;
        let mut mg_eval: i64 = 0;
        let mut eg_eval: i64 = 0;
        let mut piece: Piece;
        let mut s: i64;
        for i in 0..8 {
            for j in 0..8 {
                piece = self.board[i][j];
                s = sign(piece);
                mat_eval += s * value(piece);
                mg_eval += s * mg_weight(piece,i,j);
                eg_eval += s * eg_weight(piece,i,j);
            }
        }
        let phase = ((TOTALPHASE - self.phase) * 256 + (TOTALPHASE / 2)) / TOTALPHASE;
        sign(self.color) * (  mat_eval + ((256 - phase) * mg_eval + phase * (eg_eval + self.kings_endgame(self.color))) / 256 )
    }
}