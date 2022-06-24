use crate::model::defs::Board;

pub fn _play_puzzle(s: &str) {
    let mut game = Board::from_fen(s);
    let mut counter = 0;
    let mut score: i64;
    for _ in 0..10 {
        counter += 1;
        score = game.alpha_beta_max(-99999, 99999, 4);
        let check = game.make_move(game.best_move);
        if check.is_some() {
            if check.unwrap() { println!("Checkmate! After {counter} moves!") };
            if !check.unwrap() { println!("Stalemate! After {counter} moves!") };
            break;
        }
        println!("Current eval is {score}.");
        game.log();
    } 
}

pub const _PUZZLES: [&str; 3] = ["8/2k1R3/1pp3bp/3r2p1/PPNp4/3P1PKP/8/8 b - - 0 1",
                                "rn5r/pp3kpp/2p1R3/5p2/3P4/2B2N2/PPP3PP/2K4n w - - 1 17",
                                "4r1rk/pp4pp/2n5/8/6Q1/7R/1qPK1P1P/3R4 w - - 0 28"];

#[cfg(test)]
mod test {
    use crate::puzzles::*;

    #[test]
    pub fn all_puzzles() {
        for puzzle in _PUZZLES {
            _play_puzzle(puzzle);
        }
    }

    #[test]
    pub fn one_puzzle() {
        _play_puzzle(_PUZZLES[2]);
    }
}