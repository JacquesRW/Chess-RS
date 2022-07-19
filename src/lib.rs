pub mod model;
pub use crate::model::defs::*;

impl Board {
    pub fn new() -> Board {
        // default starting position
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn make_move(&mut self, m: Move) -> Option<bool> {
        // checks for mate and returns Some(true) (if mate), Some(false) (if stalemate), None (else)
        // should only be used when actually playing moves as adds significant overhead
        self.pseudo_move(m);
        self.check_for_mate()
    }

    pub fn engine_move(&mut self) -> Option<bool> {
        println!("AI Moving.");
        let m = self.analyse(5);
        self.make_move(m)
    }


}

#[cfg(test)]
mod puzzles; // de facto testing module now

pub const _START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const _TEST_POSITIONS: [&str;1] = ["r2qk2r/ppp4p/3p1pn1/3Pn1p1/2B1P3/2N2P1P/PP2QP2/R4RK1 w kq - 1 15"];