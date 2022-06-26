use crate::model::defs::Board;

pub fn _play_puzzle(s: &str) {
    let mut game = Board::from_fen(s);
    game.log();
    let mut counter = 0;
    for _ in 0..7 {
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

pub const _PUZZLES: [&str; 4] = ["8/2krR3/1pp3bp/42p1/PPNp4/3P1PKP/8/8 w - - 0 1", // fork in 5 - works at depth 3
                                "rn5r/pp3kpp/2p1R3/5p2/3P4/2B2N2/PPP3PP/2K4n w - - 1 17", // M7 - poor performance up until depth 6
                                "4r1rk/pp4pp/2n5/8/6Q1/7R/1qPK1P1P/3R4 w - - 0 28", // M3
                                "2r1rbk1/1R3R1N/p3p1p1/3pP3/8/q7/P1Q3PP/7K b - - 0 25"]; // M5

#[cfg(test)]
mod test {
    use crate::puzzles::*;
    use crate::model::pieces::*;
    use crate::model::helper::*;
    use crate::model::defs::*;

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

    #[test]
    pub fn pieces_kings_test() {
        let mut game = Board::from_fen("rn5r/pp3kpp/2p1R3/5p2/3P4/2B2N2/PPP3PP/2K4n w - - 1 17");
        game.log();
        println!("White Pieces:");
        for p in &game.pieces[0] {
            println!("{}", get_coords(p));
        }
        println!("Black Pieces:");
        for p in &game.pieces[1] {
            println!("{}", get_coords(p));
        }
        game.make_move(Move {target: WHITE | ROOK, orig: [5,4], dest: [5,2]} );
        game.log();
        println!("Black Pieces:");
        for p in &game.pieces[1] {
            println!("{}", get_coords(p));
        }
        println!("White Pieces:");
        for p in &game.pieces[0] {
            println!("{}", get_coords(p));
        }
        game.make_move(Move {target: BLACK | PAWN, orig: [6,1], dest: [5,2]} );
        game.log();
        println!("Black Pieces:");
        for p in &game.pieces[1] {
            println!("{}", get_coords(p));
        }
        println!("White Pieces:");
        for p in &game.pieces[0] {
            println!("{}", get_coords(p));
        }
    }
}