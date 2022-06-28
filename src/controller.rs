use crate::model::defs::*;
use crate::model::helper::*;
use crate::model::pieces::*;

impl Board {
    fn player_move(&mut self) -> Option<bool> {
        let moves = self.find_all_possible_moves();
        loop {
            let mut a = String::new();
            std::io::stdin().read_line(&mut a).unwrap();
            let coords: Vec<&str> = a.split_whitespace().collect();
            let origin = get_square(&coords[0]);
            let destination = get_square(&coords[1]);
            let m = Move { target: self.board[origin[0]][origin[1]], orig: origin, dest: destination};
            if moves.iter().any(|&i| i==m) {
                return self.make_move(m)
            }
            else {
                println!("Please enter a valid move:")
            }
        }
    }

    fn engine_move(&mut self) -> Option<bool> {
        println!("AI Moving.");
        let m = self.analyse(5);
        self.make_move(m)
    }
}


// not good atm bc i was just using it for testing
use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

#[inline(always)]
pub fn _p_v_e(fen: &str, player_color: Piece) {
    // player vs engine
    let mut game = Board::from_fen(fen);
    let mut stdout = stdout();
    stdout.queue(cursor::SavePosition).unwrap();
    stdout.execute(cursor::Hide).unwrap();
    game.log();
    let mut check: Option<bool>;
    loop {
        //stdout.execute(cursor::Hide).unwrap();
        if game.color == player_color {check = game.player_move();}
        else {check = game.engine_move()}

        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(500));
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        //stdout.execute(cursor::Show).unwrap();

        game.log();

        if check.is_some() {
            if check.unwrap() {
                println!("Checkmate, {} has won!", match game.color { WHITE => "black", BLACK => "white", _ => panic!("Invalid colour!")})
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
