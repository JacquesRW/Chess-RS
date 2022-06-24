//* ALPHA-BETA MINIMAX  */
//* CURRENTLY BRANCHES LOADS OF BOARDS */
//* GONNA TRY TO IMPLEMENT UNMAKE MOVE TO FIX THIS */

use crate::model::defs::*;
use crate::model::pieces::*;
use std::time::Instant;

impl Board {
    pub fn evaluate(&self) -> i64 {
        let mut eval: i64 = 0;
        for i in 0..8 {
            for j in 0..8 {
                eval += value(self.board[i][j])
            }
        }
        if self.color == WHITE {return eval}
        -eval
    }
    pub fn alpha_beta_max(&mut self, mut alpha: i64, beta: i64, depth_left: u8) -> i64 {
        if depth_left == 0 { return self.evaluate() }
        let moves = self.find_all_possible_moves();
        let mut check: Option<bool>;
        for m in moves {
            let mut temp = self.clone();
            check = temp.make_move(m);
            if check.is_some() { 
                if check.unwrap() {
                    self.best_move = m;
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
                self.best_move = m;
                alpha = score 
            }
        }
        return alpha
    }

    pub fn alpha_beta_min(&mut self, alpha: i64, mut beta: i64, depth_left: u8) -> i64 {
        if depth_left == 0 { return self.evaluate() }
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

    pub fn analyse(&mut self){
        let now = Instant::now();
        let eval = self.alpha_beta_max(-99999, 99999, 4);
        println!("Took {} ms to evalute position.", now.elapsed().as_millis());
        if eval > 0 {println!("Current evaluation is {eval} in favour of {}.", as_string(self.color))}
        if eval < 0 {println!("Current evaluation is {} in favour of {}.", -eval, as_string(other_colour(self.color)))}
        else {println!("Current position is equal.")}
        println!("The best move is {}", self.best_move.to_string())
    }
}