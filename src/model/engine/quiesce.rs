//* Quiescence Search. */

use crate::model::defs::*;

// value returned if forced checkmate found
pub const MAX: i64 = 999999;

// atomic count of quiescence nodes searched
use std::sync::atomic::{AtomicUsize, Ordering};
pub static QS_CALLS: AtomicUsize = AtomicUsize::new(0);
pub fn count_qs_plus() {
    QS_CALLS.fetch_add(1, Ordering::SeqCst);
}

impl Board {
    pub fn quiesce(&mut self, mut alpha: i64, beta: i64, depth_left: u8) -> i64 {
        // base eval for the current position, to be improved/worsened with these checks
        let stand_pat = self.evaluate();
        if depth_left == 0 { 
            count_qs_plus();
            return stand_pat 
        }
        let mut captures = self.find_all_possible_quiet_moves();
        // checks that game hasn't ended in mate or stalemate
        // only check if no captures available (as captures available => not mate as legal moves remain)
        if captures.is_empty() {
            // this has a huge performance impact, but is necessary for effective evaluation
            // generates all possible moves
            let check = self.check_for_mate();
            // if game over
            if check.is_some() {
                // checkmate
                if check.unwrap() {
                    count_qs_plus();
                    return -MAX
                }
                // stalemate
                return 0
            }
            return stand_pat 
        }
        // beta pruning
        // there is an argument for returning stand pat instead of beta
        // TESTING required
        if stand_pat >= beta { 
            count_qs_plus();
            return beta 
        }
        // delta pruning, no need to search if up a queen's worth of material 
        // prevents excessive branching in clearly winning positions
        // (when move quality is not as important)
        if stand_pat < alpha - 900 {
            count_qs_plus();
            return alpha
        }
        // improving alpha bound
        if alpha < stand_pat {
            alpha = stand_pat;
        }
        // no need to waste time sorting moves on depth 1 as every one should be evaluated
        // not as much effect as seen for negamax
        // probably because far fewer moves
        if depth_left != 1 {
            // MVV-LVA move ordering
            captures.sort_by_key(|a| a.score(self.board[a.dest[0]][a.dest[1]]));
            captures.reverse();
        }
        // same as for negamax
        for m in captures {
            let pen_kings = self.kings;
            let pen_castle = self.castle;
            let pen_move = self.last_move;
            let capture = self.board[m.dest[0]][m.dest[1]];
            self.pseudo_move(m);
            let score = -self.quiesce(-beta, -alpha, depth_left-1);
            self.unmake_move(m, pen_move, pen_castle, capture, pen_kings);
            if score >= beta { return beta }
            if score > alpha { alpha = score }
        }
        count_qs_plus();
        alpha
    }
}