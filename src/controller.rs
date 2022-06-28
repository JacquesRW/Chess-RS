use crate::model::defs::*;
use crate::model::helper::*;
use crate::model::pieces::*;

impl Board {
    fn player_move(&mut self) -> Option<bool> {
        // allowable moves
        let moves = self.find_all_possible_moves();
        loop {
            // taking input from player
            let mut a = String::new();
            std::io::stdin().read_line(&mut a).unwrap();
            // collecting input into a vector split at whitespace
            let coords: Vec<&str> = a.split_whitespace().collect();
            // getting origin of move
            let origin = get_square(&coords[0]);
            // getting destination of move
            let destination = get_square(&coords[1]);
            // getting target of move
            let m = Move { target: self.board[origin[0]][origin[1]], orig: origin, dest: destination};
            // validating input
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
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

#[inline(always)]
pub fn _p_v_e(fen: &str, player_color: Piece) {
    // needed things
    let mut stdout = stdout();
    let mut check: Option<bool>;
    // player vs engine
    let mut game = Board::from_fen(fen);
    // setting restore point for cursor and hiding it
    stdout.queue(cursor::SavePosition).unwrap();
    stdout.execute(cursor::Hide).unwrap();
    // displaying initial board
    game.log();
    // loop of moves
    loop {
        // playing move
        if game.color == player_color {check = game.player_move();}
        else {check = game.engine_move()}
        // clearing terminal
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        // displaying board
        game.log();
        // checking if game ends
        if check.is_some() {
            // checkmate
            if check.unwrap() {
                println!("Checkmate, {} has won!", match game.color { WHITE => "black", BLACK => "white", _ => panic!("Invalid colour!")})
            }
            // stalemate
            if !check.unwrap() {
                println!("Stalemate!")
            }
            break;
        }
    }
    // prevents terminal from closing instantly
    let mut a = String::new();
    let end = std::io::stdin().read_line(&mut a).unwrap();
    println!("{}", end);
}
