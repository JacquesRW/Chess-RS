use crate::model::defs::Board;

#[inline(always)]
pub fn _play_puzzle(s: &str) {
    let mut game = Board::from_fen(s);
    game.log();
    let mut counter = 0;
    for _ in 0..8 {
        counter += 1;
        let m = game.analyse(5);
        let check = game.make_move(m);
        game.log();
        if check.is_some() {
            if check.unwrap() { println!("Checkmate! After {counter} moves!") };
            if !check.unwrap() { println!("Stalemate! After {counter} moves!") };
            break;
        }
    } 
}

pub const _PUZZLES: [&str; 4] = ["8/2krR3/1pp3bp/42p1/PPNp4/3P1PKP/8/8 w - - 0 1",
                                "rn5r/pp3kpp/2p1R3/5p2/3P4/2B2N2/PPP3PP/2K4n w - - 1 17",
                                "4r1rk/pp4pp/2n5/8/6Q1/7R/1qPK1P1P/3R4 w - - 0 28",
                                "2r1rbk1/1R3R1N/p3p1p1/3pP3/8/q7/P1Q3PP/7K b - - 0 25"]; 

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
        _play_puzzle(_PUZZLES[1]);
    }
}