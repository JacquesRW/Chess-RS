#[cfg(test)]
mod test {
    use crate::model::defs::{Move,Board, Piece};
    use crate::model::pieces::*;

    #[test]
    fn castling_kingside() {
        println!("Testing kingside castling.");
        let mut game = Board::new();
        game.make_move(Move { target: WHITE | PAWN, orig: [1,4], dest: [3,4]});
        game.make_move(Move { target: BLACK | PAWN, orig: [6,4], dest: [4,4]});
        game.make_move(Move { target: WHITE | BISHOP, orig: [0,5], dest: [1,4]});
        game.make_move(Move { target: BLACK | BISHOP, orig: [7,5], dest: [6,4]});
        game.make_move(Move { target: WHITE | KNIGHT, orig: [0,6], dest: [2,5]});
        game.make_move(Move { target: BLACK | KNIGHT, orig: [7,6], dest: [5,5]});
        game.log();
        game.make_move(Move { target: WHITE | KING, orig: [0,4], dest: [0,6]});
        game.log();
        game.make_move(Move { target: BLACK | KING, orig: [7,4], dest: [7,6]});
        game.log();
    }
    #[test]
    fn castling_queenside() {
        println!("Testing queenside castling.");
        let mut game = Board::new();
        game.make_move(Move { target: WHITE | PAWN, orig: [1,3], dest: [3,3]});
        game.make_move(Move { target: BLACK | PAWN, orig: [6,3], dest: [4,3]});
        game.make_move(Move { target: WHITE | BISHOP, orig: [0,2], dest: [2,4]});
        game.make_move(Move { target: BLACK | BISHOP, orig: [7,2], dest: [5,4]});
        game.make_move(Move { target: WHITE | KNIGHT, orig: [0,1], dest: [2,2]});
        game.make_move(Move { target: BLACK | KNIGHT, orig: [7,1], dest: [5,2]});
        game.make_move(Move { target: WHITE | QUEEN, orig: [0,3], dest: [1,3]});
        game.make_move(Move { target: BLACK | QUEEN, orig: [7,3], dest: [6,3]});
        game.log();
        game.make_move(Move { target: WHITE | KING, orig: [0,4], dest: [0,2]});
        game.log();
        game.make_move(Move { target: BLACK | KING, orig: [7,4], dest: [7,2]});
        game.log();
    }
    #[test]
    fn white_en_passant() {
        println!("Testing en passant.");
        let mut game = Board::new();
        game.make_move(Move { target: WHITE | PAWN, orig: [1,4], dest: [3,4]});
        game.make_move(Move { target: BLACK | PAWN, orig: [6,4], dest: [5,4]});
        game.make_move(Move { target: WHITE | PAWN, orig: [3,4], dest: [4,4]});
        game.make_move(Move { target: BLACK | PAWN, orig: [6,3], dest: [4,3]});
        game.log();
        game.make_move(Move { target: WHITE | PAWN, orig: [4,4], dest: [5,3]});
        game.log();
    }
    #[test]
    fn black_en_passant() {
        println!("Testing en passant.");
        let mut game = Board::new();
        game.make_move(Move { target: WHITE | PAWN, orig: [1,4], dest: [2,4]});
        game.make_move(Move { target: BLACK | PAWN, orig: [6,4], dest: [4,4]});
        game.make_move(Move { target: WHITE | PAWN, orig: [1,2], dest: [2,2]});
        game.make_move(Move { target: BLACK | PAWN, orig: [4,4], dest: [3,4]});
        game.make_move(Move { target: WHITE | PAWN, orig: [1,3], dest: [3,3]});
        game.log();
        game.make_move(Move { target: BLACK | PAWN, orig: [3,4], dest: [2,3]});
        game.log();
    }

    #[test]
    fn movegen_checks() {
        println!("Checking if move generation under check is implemented properly.");
        let mut game = Board::new();
        game.make_move(Move { target: WHITE | PAWN, orig: [1,4], dest: [3,4]});
        game.make_move(Move { target: BLACK | PAWN, orig: [6,4], dest: [4,4]});
        game.make_move(Move { target: WHITE | QUEEN, orig: [0,3], dest: [4,7]});
        game.log();
        let test1 = game.selection_possible_moves([6,5]);
        println!("Possible Moves: Expect none.");
        for m in test1 {
            m.log()
        }
        game.make_move(Move { target: BLACK | PAWN, orig: [6,3], dest: [5,3]});
        game.make_move(Move { target: WHITE | QUEEN, orig: [4,7], dest: [4,4]});
        game.log();
        let test2 = game.find_all_possible_moves();
        println!("Possible Moves: Expect 6.");
        for m in test2 {
            m.log()
        }
    }

    #[test]
    fn speed_test() {
        use rand::thread_rng;
        use rand::seq::SliceRandom;
        use std::time::Instant;
        println!("Performing speed test on move generation for 50500 random moves.");
        let now = Instant::now();
        for _ in 0..10 {
            let mut game = Board::new();
            let mut moves: Vec<Move>;
            let mut m: Move;
            for _ in 0..5 {
                for _ in 0..10 {
                    moves = game.find_all_possible_moves();
                    m = *moves.choose(&mut thread_rng()).unwrap();
                    game.make_move(m);
                }
                //game.log();
                for _ in 0..1000 {
                    game.find_all_possible_moves();
                }
            }
        }
        println!("Took {} ms", now.elapsed().as_millis())
    }

    #[test]
    fn error_free_movegen_test() {
        use rand::thread_rng;
        use rand::seq::SliceRandom;
        println!("Performing 50 random moves.");
        let mut game = Board::new();
        let mut moves: Vec<Move>;
        let mut m: Move;
        for i in 0..100 {
            moves = game.find_all_possible_moves();
            m = *moves.choose(&mut thread_rng()).unwrap();
            game.make_move(m);
            println!("{i}");
            game.log();
        }
    }
}