mod model;
use crate::model::defs::{Move,Board};
use crate::model::helper::*;
use crate::model::pieces::*;
// THIS USES [COLUMN, ROW] CONVENTION JOHN
// I BETTER NOT BE SEEING NO DUMBASS STUFF

#[cfg(test)]
mod tests;

fn main() {
    println!("{KING}");
    println!("{BLACK}");
    let pc = BLACK | KING;
    println!("{pc}");
    let mut game = Board::new();
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
    }
}
