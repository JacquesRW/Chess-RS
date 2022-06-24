mod model;
use crate::model::defs::{Move,Board};
use crate::model::helper::*;
// THIS USES [COLUMN, ROW] CONVENTION JOHN
// I BETTER NOT BE SEEING NO DUMBASS STUFF

#[cfg(test)]
mod tests;
mod puzzles;

fn main() {
    let mut game = Board::from_fen("r2qk2r/ppp4p/3p1pn1/3Pn1p1/2B1P3/2N2P1P/PP2QP2/R4RK1 w kq - 1 15");
    game.log();

    loop {
        let mut a = String::new();
        println!("Piece to move:");
        let b1 = std::io::stdin().read_line(&mut a).unwrap();
        if b1 != 4 {break}
        let origin = get_square(&a);
        game.get_piece_selection(origin);

        let moves = game.selection_possible_moves(origin);
        Move::print_destinations(&moves);

        let mut a2 = String::new();
        println!("Move to:");
        let b2 = std::io::stdin().read_line(&mut a2).unwrap();
        if b2 != 4 {break}
        let destination = get_square(&a2);

        let m = Move::new(game.board[origin[0]][origin[1]],origin,destination);
        if moves.iter().any(|&i| i==m) {
            let check = game.make_move(m);
            game.log();
            if !check.is_none() {
                if check.unwrap() {
                    println!("Checkmate!");
                    break;
                }
                else {
                    println!("Stalemate!");
                    break;
                }
            }
        }
        else {break}

        game.analyse(5);
        let check = game.make_move(game.best_move);
        game.log();
        if check.is_some() {
            if check.unwrap() { println!("Checkmate!") };
            if !check.unwrap() { println!("Stalemate!") };
            break;
        }
    }
}
