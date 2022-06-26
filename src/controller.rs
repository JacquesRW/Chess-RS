use crate::model::defs::*;
use crate::model::helper::*;
use crate::model::pieces::*;
use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

// not good atm bc i was just using it for testing

#[inline(always)]
pub fn _p_v_e(fen: &str, player_color: Piece) {
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
        thread::sleep(time::Duration::from_millis(500));
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(cursor::Show).unwrap();

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

#[inline(always)]
pub fn _e_v_e(fen: &str) {
    let mut game = Board::from_fen(fen);
    let mut stdout = stdout();
    stdout.queue(cursor::SavePosition).unwrap();
    game.log();
    let mut check: Option<bool>;
    loop {
        stdout.execute(cursor::Hide).unwrap();
        check = game.ai_move();

        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(1000));
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(cursor::Show).unwrap();

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

pub fn _p_v_p(fen: &str) {
    let mut game = Board::from_fen(fen);
    let mut stdout = stdout();
    stdout.queue(cursor::SavePosition).unwrap();
    game.log();
    let mut check: Option<bool>;
    loop {
        stdout.execute(cursor::Hide).unwrap();
        check = game.player_move();

        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(cursor::Show).unwrap();

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

impl Board {
    fn player_move(&mut self) -> Option<bool> {
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).unwrap();
        let coords: Vec<&str> = a.split_whitespace().collect();
        let origin = get_square(&coords[0]);
        let destination = get_square(&coords[1]);
        let moves = self.selection_possible_moves(origin);

        let m = Move::new(self.board[origin[0]][origin[1]],origin,destination);
        if moves.iter().any(|&i| i==m) {
            return self.make_move(m)
        }
        else { panic!("Not a valid move!") }
    }

    fn ai_move(&mut self) -> Option<bool> {
        println!("AI Moving.");
        let m = self.analyse(5);
        self.make_move(m)
    }
}
