use crate::model::defs::*;
use crate::model::pieces::*;

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
                    return -999
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
                    return 999
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
}