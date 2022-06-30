//* NegaMax Algorithm.  */

use crate::model::defs::*;
use crate::model::pieces::*;
use std::time::Instant;

use std::sync::atomic::{AtomicUsize, Ordering};
// atomic call counts because negamax already returns a value + easy to just remove than one written into it
static NM_CALLS: AtomicUsize = AtomicUsize::new(0);
use crate::model::engine::quiesce::{QS_CALLS, MAX};
// add 1 to node counts
fn count_plus() {
    NM_CALLS.fetch_add(1, Ordering::SeqCst);
}

impl Board {
    fn negamax(&mut self, mut alpha: i64, beta: i64, depth_left: u8) -> i64 {
        // end of recursive depth of search
        if depth_left == 0 {
            count_plus();
            // quiescence search of only takes, promotions and checks to mitigate horizon effect
            return self.quiesce(alpha, beta, 4) 
        }
        let mut moves = self.find_all_possible_moves();
        // checks if the game is over in this position, returns max if mate and 0 for stalemate
        // NOTE this comes after quiescence search because if depth 0 && chechmate/stalemate, it will be picked up
        // but if the other way round it will generate all legal moves for non-ending positions and slow algorithm down
        if moves.is_empty() {
            count_plus();
            // checkmate
            if self.check_for_check_static(self.kings[colour_to_index(self.color)], self.color) {
                return -MAX
            }
            // stalemate
            return 0
        }
        // sorts the list according to MVV-LVA
        // PLANNED optimisation - no built in functions serve better
        // POTENTIAL rework to not order whole list
        // no sort on depth 1 because testing indicated it took longer
        if depth_left != 1 {
            moves.sort_by_key(|a| a.score(self.board[a.dest[0]][a.dest[1]]));
            moves.reverse();
        }
        // going through legal moves
        for m in moves {
            // relevant info needed to unmake move
            // not stored in Board struct for memory reasons, as the Board struct is used elsewhere
            // where these fields are not relevant
            let pen_kings = self.kings;
            let pen_castle = self.castle;
            let pen_move = self.last_move;
            let capture = self.board[m.dest[0]][m.dest[1]];
            // pseudo-move does not check for checkmates, done above instead
            // avoids double calculating moves
            self.pseudo_move(m);
            // recursively calls itself for the next set of possible moves
            // based on principle that max(a,b) = -min(-a,-b)
            // i.e best outcome for other player is worst for you
            let score = -self.negamax(-beta, -alpha, depth_left - 1);
            self.unmake_move(m, pen_move, pen_castle, capture, pen_kings);
            // beta pruning
            if score >= beta { return beta }
            // improve alpha bound
            if score > alpha { alpha = score }
        }
        // if no beta pruning, return alpha
        count_plus();
        alpha
    }

    #[inline(always)]
    fn negamax_root(&mut self, mut alpha: i64, beta: i64, depth_left: u8, move_list: Vec<ScoredMove>) -> Vec<ScoredMove> {
        // vector of (move, score)
        // uses scored move_list from previous depth search (in iterative deepening)
        let mut new_move_list: Vec<ScoredMove> = Vec::new();
        // equivalent negamax to above but records the scores of each move
        for sm in move_list {
            let mo = sm.m;
            let pen_kings = self.kings;
            let pen_castle = self.castle;
            let pen_move = self.last_move;
            let capture = self.board[mo.dest[0]][mo.dest[1]];
            self.pseudo_move(mo);
            let score = -self.negamax(-beta, -alpha, depth_left - 1);
            self.unmake_move(mo, pen_move, pen_castle, capture, pen_kings);
            if score > alpha {
                alpha = score - 1;
            }
            new_move_list.push(ScoredMove {m: mo, s: score} )
        }
        // sorts list of moves by score 
        // performed once per search so no sense in making it more optimal
        new_move_list.sort_by(|a, b| a.s.cmp(&b.s));
        new_move_list.reverse();
        // returns new_move_list for use in next iteration or to take the top move out
        new_move_list
    }

    #[inline(always)]
    pub fn analyse(&mut self, depth: u8) -> Move {
        // an iterative deepening search
        // PLANNED Zobrist hashing for more performance
        let now = Instant::now();
        // all possible moves to seed the search
        let moves = self.find_all_possible_moves();
        // creating the initial scored move list with all scores set to 0
        let mut move_list: Vec<ScoredMove> = Vec::new();
        for mo in moves {
            move_list.push(ScoredMove { m: mo, s: 0 });
        }
        // loop of iterative deepening, up to preset max depth
        for d in 1..(depth+1) {
            move_list = self.negamax_root(-9999999, 9999999, d, move_list);
            // if a forced checkmate is found the search ends obviously
            if move_list[0].s == MAX {
                break;
            }
        }
        println!("Took {} ms to evalute {:?} normal nodes and {:?} quiescent nodes", now.elapsed().as_millis(), NM_CALLS.load(Ordering::SeqCst), QS_CALLS.load(Ordering::SeqCst));
        // reset atomic counters to zero before next position analysed
        NM_CALLS.store(0, Ordering::SeqCst);
        QS_CALLS.store(0, Ordering::SeqCst);
        println!("Current eval is {}.", match self.color { BLACK => -move_list[0].s, WHITE => move_list[0].s, _ => panic!("Invalid colour!")});
        move_list[0].m
    }
}