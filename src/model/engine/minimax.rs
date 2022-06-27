//* ALPHA-BETA MINIMAX  */

use crate::model::defs::*;
//use crate::model::moves::*;
use crate::model::pieces::*;
use std::time::Instant;

const MAX: i64 = 999999;

use std::sync::atomic::{AtomicUsize, Ordering};

static TOTAL_CALLS: AtomicUsize = AtomicUsize::new(0);

fn function_to_count() {
    TOTAL_CALLS.fetch_add(1, Ordering::SeqCst);
}

impl Board {
    // alpha-beta pruning minimax method
    // PLANNED introduction of quiescence search rather than eval
    // POTENTIAL refactor to negamax
    #[inline(always)]
    fn alpha_beta_max(&mut self, mut alpha: i64, beta: i64, depth_left: u8) -> i64 {
        if depth_left == 0 { 
            function_to_count();
            return self.evaluate() 
        }
        let moves = self.find_all_possible_moves();
        let mut check: Option<bool>;
        for m in moves {
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
            let score = self.alpha_beta_min(alpha, beta, depth_left - 1);
            self.unmake_move(m, pen_move, pen_castle, capture);
            if score >= beta { 
                return beta 
            }
            if score > alpha { 
                alpha = score 
            }
        }
        return alpha
    }
    #[inline(always)]
    fn alpha_beta_min(&mut self, alpha: i64, mut beta: i64, depth_left: u8) -> i64 {
        if depth_left == 0 { 
            function_to_count();
            return -self.evaluate() 
        }
        let moves = self.find_all_possible_moves();
        let mut check: Option<bool>;
        for m in moves {
            let pen_castle = self.castle;
            let pen_move = self.last_move;
            let capture = self.board[m.dest[0]][m.dest[1]];
            check = self.make_move(m);
            if check.is_some() { 
                if check.unwrap() {
                    self.unmake_move(m, pen_move, pen_castle, capture);
                    return -MAX
                }
                if !check.unwrap() {
                    self.unmake_move(m, pen_move, pen_castle, capture);
                    return 0
                }
            }
            let score = self.alpha_beta_max(alpha, beta, depth_left - 1);
            self.unmake_move(m, pen_move, pen_castle, capture);
            if score <= alpha { 
                return alpha 
            }
            if score < beta { 
                beta = score 
            }
        }
        return beta
    }

    #[inline(always)]
    fn move_list_ab_max(&mut self, mut alpha: i64, beta: i64, depth_left: u8, move_list: Vec<ScoredMove>) -> Vec<ScoredMove> {
        // takes in an ordered vector of moves and calls alpha-beta minimax on those moves
        // aims to increase amount of branches pruned
        let mut new_move_list: Vec<ScoredMove> = Vec::new();
        let mut check: Option<bool>;
        let colo = self.color;
        for sm in move_list {
            let mo = sm.m;
            let pen_castle = self.castle;
            let pen_move = self.last_move;
            let capture = self.board[mo.dest[0]][mo.dest[1]];
            let mut score: i64 = 0;
            check = self.make_move(mo);
            if check.is_some() { 
                if check.unwrap() {
                    self.unmake_move(mo, pen_move, pen_castle, capture);
                    score = MAX;
                }
                if !check.unwrap() {
                    self.unmake_move(mo, pen_move, pen_castle, capture);
                    score = 0;
                }
            }
            else {
                score = self.alpha_beta_min(alpha, beta, depth_left - 1);
            }
            self.unmake_move(mo, pen_move, pen_castle, capture);
            if score > alpha {
                alpha = score - 1;
            }
            self.color = colo;
            new_move_list.push(ScoredMove {m: mo, s: score} )
        }
        new_move_list.sort_by(|a, b| a.s.cmp(&b.s));
        new_move_list.reverse();
        return new_move_list
    }

    #[inline(always)]
    pub fn analyse(&mut self, depth: u8) -> Move {
        // an iterative deepening search
        // PLANNED Zobrist hashing for more performance
        let now = Instant::now();
        let mut move_list: Vec<ScoredMove> = Vec::new();
        let moves = self.find_all_possible_moves();
        for mo in moves {
            move_list.push(ScoredMove { m: mo, s: 0 });
        }
        for d in 1..(depth+1) {
            //println!("Depth {d}.");
            move_list = self.move_list_ab_max(-9999999, 9999999, d, move_list);
            //_output_move_list(&move_list);
            if move_list[0].s == MAX {
                break;
            }
        }
        println!("Took {} ms to evalute {:?} positions.", now.elapsed().as_millis(), TOTAL_CALLS.load(Ordering::SeqCst));
        TOTAL_CALLS.store(0, Ordering::SeqCst);
        println!("Current eval is {}.", match self.color { BLACK => -move_list[0].s, WHITE => move_list[0].s, _ => panic!("Invalid colour!")});
        move_list[0].m
    }
}