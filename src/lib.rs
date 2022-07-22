pub mod model;
pub use crate::model::defs::*;

// top level commands exposed by this library
// should be able to get by with just these
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

    pub fn engine_move(&mut self, depth: u8, quiesce_depth: u8, move_log: bool) -> Option<bool> {
        // making an AI move
        println!("AI Moving.");
        let m = self.analyse(depth, quiesce_depth);
        if move_log { m.log() }
        self.make_move(m)
    }


}

#[cfg(test)]
mod test; // de facto testing module now
