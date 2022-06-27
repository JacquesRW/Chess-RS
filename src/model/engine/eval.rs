use crate::model::defs::*;
use crate::model::pieces::*;
use crate::model::engine::constants::*;
const MAX: i64 = 999999;

#[inline(always)]
fn value(pc: Piece) -> i64 {
    match name(pc) { 
        EMPTY => 0,
        PAWN => 100,
        BISHOP => 330,
        KNIGHT => 320,
        ROOK => 500,
        QUEEN => 900,
        KING => 20000,
        _ => panic!("Not a valid piece!")
    }
}

#[inline(always)]
fn sign(piece: Piece) -> i64 {
    match colour(piece) {
        WHITE => 1, 
        BLACK => -1, 
        EMPTY => 0, 
        _=> panic!("Not valid piece!")}
}

#[inline(always)]
fn weight(piece: Piece, i: usize, j: usize) -> i64 {
    match colour(piece) {
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
        }
}

impl Board {
    #[inline(always)]
    pub fn evaluate(&self) -> i64 {
        // PLANNED transition to endgame values
        let mut eval: i64 = 0;
        let mut piece: Piece;
        for i in 0..8 {
            for j in 0..8 {
                piece = self.board[i][j];
                eval += sign(piece) * (value(piece) + weight(piece,i,j));
                }
        }
        eval * sign(self.color)
    }

    pub fn quiesce(&mut self, mut alpha: i64, beta: i64) -> i64 {
        let stand_pat = self.evaluate();
        if stand_pat >= beta {return beta}
        if alpha < stand_pat {
            alpha = stand_pat;
        }
        let captures = self.find_all_possible_takes();
        let mut check: Option<bool>;
        for m in captures {
            let pen_castle = self.castle;
            let pen_move = self.last_move;
            let capture = self.board[m.dest[0]][m.dest[1]];
            check = self.make_move(m);
            if check.is_some() { 
                if check.unwrap() {
                    self.unmake_move(m, pen_move, pen_castle, capture);
                    return MAX
                }
                if !check.unwrap() {
                    self.unmake_move(m, pen_move, pen_castle, capture);
                    return 0
                }
            }
            let score = -self.quiesce(-beta, -alpha);
            self.unmake_move(m, pen_move, pen_castle, capture);
            if score >= beta { return beta }
            if score > alpha { alpha = score }
        }
        alpha
    }
}