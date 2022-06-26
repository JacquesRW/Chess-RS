#[cfg(test)]
mod test {
    use crate::model::defs::{Move,Board};
    use crate::model::pieces::*;
    use crate::model::engine::*;


    #[test]
    fn checkmate() {
        println!("Testing checkmate.");
        let mut game = Board::_new();
        game.make_move(Move { target: WHITE | PAWN, orig: [1,4], dest: [3,4]});
        game.make_move(Move { target: BLACK | PAWN, orig: [6,4], dest: [4,4]});
        game.make_move(Move { target: WHITE | BISHOP, orig: [0,5], dest: [3,2]});
        game.make_move(Move { target: BLACK | PAWN, orig: [6,0], dest: [5,0]});
        game.log();
        game.analyse(4);
        game.make_move(Move { target: WHITE | QUEEN, orig: [0,3], dest: [2,5]});
        game.log();
        game.analyse(4);
        game.make_move(Move { target: BLACK | PAWN, orig: [6,1], dest: [4,1]});
        game.log();
        game.analyse(4);
        let check = game.make_move(Move { target: WHITE | QUEEN, orig: [2,5], dest: [6,5]});
        if check.unwrap() {println!("Checkmate!")}
        game.log();
    }

    #[test]
    fn test_ai() {
        println!("Testing ai playing against each other.");
        let mut game = Board::_new();
        game.log();
        for _ in 0..100 {
            game.analyse(4);
            let check = game.make_move(game.best_move);
            if check.is_some() {
                if check.unwrap() {println!("Checkmate!")
                }   
                if !check.unwrap() {println!("Stalemate!")}
            }
            game.log();
        }
    }
}