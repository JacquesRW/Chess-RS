use crate::model::defs::Board;

#[inline(always)]
pub fn _play_puzzle(s: &str) {
    let mut game = Board::from_fen(s);
    game.log();
    let mut counter = 0;
    for _ in 0..8 {
        counter += 1;
        let m = game.analyse(6);
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
    use crate::model::{defs::*,pieces::*};

    #[test]
    fn all_puzzles() {
        for puzzle in _PUZZLES {
            _play_puzzle(puzzle);
        }
    }

    #[test]
    fn one_puzzle() {
        _play_puzzle(_PUZZLES[3]);
    }
    #[test]
    fn overflow() {
        let mut game = Board::_new();  
        game.make_move( Move { target: WHITE | PAWN, orig: [1,1], dest: [3,1]});
        game.log();
        game.make_move( Move { target: BLACK | PAWN, orig: [6,0], dest: [5,0]});
        game.log();
        game.make_move( Move { target: WHITE | PAWN, orig: [3,1], dest: [4,1]});
        game.log();
        game.make_move( Move { target: BLACK | PAWN, orig: [6,2], dest: [4,2]});
        game.log();
        game.make_move( Move { target: WHITE | PAWN, orig: [4,1], dest: [5,1]});
        game.log();
        let moves = game.find_all_possible_moves();
        let mut count = 0;
        for m in moves {
            m.log();
            count += 1;
        }
        println!("{count}");
        println!("Calculating one step positions.");
        let positions = game._perft(1);
        println!("{positions}");
    }
}