//* Implementation of methods for the Move struct. */
use crate::model::defs::{Piece, Square, Move};
use crate::model::helper::*;
use crate::model::pieces::*;

impl Move {

    pub fn null() -> Move {
        Move {target: EMPTY, orig: [0,0], dest: [0,0]}
    }

    pub fn new(piece: Piece, origin: Square, destination: Square) -> Move {
        Move {target: piece, orig: origin, dest: destination}
    }

    pub fn log(&self) {
        match self {
            Move { target: WHITE | KING , orig: [0,4], dest: [0,2] } => println!("White castles queenside."),
            Move { target: WHITE | KING , orig: [0,4], dest: [0,6] } => println!("White castles kingside."),
            Move { target: BLACK | KING , orig: [7,4], dest: [7,2] } => println!("Black castles queenside."),
            Move { target: BLACK | KING , orig: [7,4], dest: [7,6] } => println!("Black castles kingside."),
            _ => println!("{}, {} to {}", self.target.to_string(), get_coords(self.orig), get_coords(self.dest))
        }
    }

    pub fn print_destinations(moves: &Vec<Self>) {
        let mut message = String::from("Possible Destinations: ");
        for m in moves {
            message += &get_coords(m.dest);
            message += &String::from(" ");
        }
        println!("{message}")
    }

}