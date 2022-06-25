use crate::model::defs::*;
use crate::model::helper::*;
use crate::model::pieces::*;
use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

#[inline(always)]
pub fn p_v_e(fen: &str, player_color: Piece) {
    let mut game = Board::from_fen(fen);
    let mut stdout = stdout();
    stdout.queue(cursor::SavePosition).unwrap();
    game.log();
    let mut check: Option<bool>;
    loop {
        stdout.execute(cursor::Hide).unwrap();
        if game.color == player_color {check = game.player_move();}
        else {check = game.ai_move()}

        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(cursor::Show).unwrap();

        game.log();

        if check.is_some() {
            if check.unwrap() {
                println!("Checkmate, {} has won!", other_colour(game.color))
            }
            if !check.unwrap() {
                println!("Stalemate!")
            }
            break;
        }
    }
    let mut a = String::new();
    let end = std::io::stdin().read_line(&mut a).unwrap();
    println!("{}", end);
}

impl Board {
    fn player_move(&mut self) -> Option<bool> {
        let mut a = String::new();
        println!("Piece to move:");
        std::io::stdin().read_line(&mut a).unwrap();
        let origin = get_square(&a);
        self.get_piece_selection(origin);

        let moves = self.selection_possible_moves(origin);
        Move::print_destinations(&moves);

        let mut a2 = String::new();
        println!("Move to:");
        std::io::stdin().read_line(&mut a2).unwrap();
        let destination = get_square(&a2);

        let m = Move::new(self.board[origin[0]][origin[1]],origin,destination);
        if moves.iter().any(|&i| i==m) {
            return self.make_move(m)
        }
        else { panic!("Not a valid move!") }
    }

    fn ai_move(&mut self) -> Option<bool> {
        println!("AI Moving.");
        self.analyse(5);
        self.make_move(self.best_move)
    }
}
