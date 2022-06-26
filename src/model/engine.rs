//* ALPHA-BETA MINIMAX  */
//* CURRENTLY BRANCHES LOADS OF BOARDS */
//* GONNA TRY TO IMPLEMENT UNMAKE MOVE TO FIX THIS */

use crate::model::defs::*;
use crate::model::moves::*;
use crate::model::pieces::*;
use std::collections::HashMap;
use std::time::Instant;

impl Board {
    #[inline(always)]
    fn evaluate(&self) -> i64 {
        // currently on material eval
        // PLANNED refactor to pieces field in Board struct for speed
        // PLANNED adding positional benefits
        let mut eval: i64 = 0;
        for i in 0..8 {
            for j in 0..8 {
                eval += value(self.board[i][j])
            }
        }
        if self.color == WHITE {return eval}
        -eval
    }

    // alpha-beta pruning minimax method
    // POTENTIAL refactor to negamax
    fn alpha_beta_max(&mut self, mut alpha: i64, beta: i64, depth_left: u8) -> i64 {
        if depth_left == 0 { return self.evaluate() }
        let moves = self.find_all_possible_moves();
        let mut check: Option<bool>;
        for m in moves {
            let mut temp = self.clone();
            check = temp.make_move(m);
            if check.is_some() { 
                if check.unwrap() {
                    return 999
                }
                if !check.unwrap() {
                    return 0
                }
            }
            let score = temp.alpha_beta_min(alpha, beta, depth_left - 1);
            if score >= beta { 
                return beta 
            }
            // self.unmake_move(m);
            if score > alpha { 
                alpha = score 
            }
        }
        return alpha
    }

    fn alpha_beta_min(&mut self, alpha: i64, mut beta: i64, depth_left: u8) -> i64 {
        if depth_left == 0 { return -self.evaluate() }
        let moves = self.find_all_possible_moves();
        let mut check: Option<bool>;
        for m in moves {
            let mut temp = self.clone();
            check = temp.make_move(m);
            if check.is_some() { 
                if check.unwrap() {
                    return -999
                }
                if !check.unwrap() {
                    return 0
                }
            }
            let score = temp.alpha_beta_max(alpha, beta, depth_left - 1);
            if score <= alpha { 
                return alpha 
            }
            // self.unmake_move(m);
            if score < beta { 
                beta = score 
            }
        }
        return beta
    }


    #[inline(always)]
    fn move_list_ab_max(&mut self, mut alpha: i64, beta: i64, depth_left: u8, move_list: Vec<ScoredMove>) -> Vec<ScoredMove> {
        // takes in an ordered (hopefully) list and calls alpha-beta minimax on those moves
        // aims to increase amount of branches pruned
        let mut new_move_list: Vec<ScoredMove> = Vec::new();
        let mut check: Option<bool>;
        for m in move_list {
            let mut score: i64 = 0;
            let mut temp = self.clone();
            check = temp.make_move(m.m);
            if check.is_some() { 
                if check.unwrap() {
                    score = 999;
                }
                if !check.unwrap() {
                    score = 0;
                }
            }
            else {
                score = temp.alpha_beta_min(alpha, beta, depth_left - 1);
            }
            // WORK OUT ISSUE HERE
            //if score >= alpha {
            //    alpha = score
            //}
            new_move_list.push(ScoredMove {m: m.m, s: score} )
        }
        new_move_list.sort_by(|a, b| a.s.cmp(&b.s));
        new_move_list.reverse();
        return new_move_list
    }

    #[inline(always)]
    pub fn analyse(&mut self, depth: u8) -> Move {
        // currently an iterative deepening search w/ minimax
        // PLANNED Zobrist hashing for more performance
        let now = Instant::now();
        let mut move_list: Vec<ScoredMove> = Vec::new();
        let moves = self.find_all_possible_moves();
        for mo in moves {
            move_list.push(ScoredMove { m: mo, s: 0 });
        }
        for d in 1..(depth+1) {
            move_list = self.move_list_ab_max(-99999, 99999, d, move_list);
            //output_move_list(&move_list);
            if move_list[0].s == 999 {
                break;
            }
            println!("Depth {d} took {}ms.", now.elapsed().as_millis());
        }
        println!("Took {} ms to evalute position.", now.elapsed().as_millis());
        move_list[0].m
    }

    pub fn _old_analyse(&mut self, depth: u8) {
        let now = Instant::now();
        self.alpha_beta_max(-99999, 99999, depth);
        println!("Took {} ms to evalute position.", now.elapsed().as_millis());
    }
}