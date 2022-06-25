use crate::model::defs::*;
use crate::model::helper::*;
use crate::model::pieces::*;

#[inline(always)]
pub fn p_v_e(fen: &str, player_color: Piece) {
    let mut game = Board::from_fen(fen);
    game.log();

    loop {
        let check: Option<bool>;
        if game.color == player_color {
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
                check = game.make_move(m);
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
            else { println!("Not a valid move!") }
        }
        else if game.color == other_colour(player_color) {
            game.analyse(4);
            check = game.make_move(game.best_move);
        }
        else { check = None; }

        game.log();
        if check.is_some() {
            if check.unwrap() { println!("Checkmate!") };
            if !check.unwrap() { println!("Stalemate!") };
            break;
        }
    }
}
}