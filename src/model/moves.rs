//* Implementation of methods for the Move struct. */
use crate::model::defs::{Piece, Square, Move};
use crate::model::helper::*;

impl Move {

    pub fn null() -> Move {
        Move {target: Piece::empty(), orig: [0,0], dest: [0,0]}
    }
    pub fn new(piece: Piece, origin: Square, destination: Square) -> Move {
        Move {target: piece, orig: origin, dest: destination}
    }

    pub fn log(&self) {
        match self {
            Move { target: Piece{piece: 'K', color: 1} , orig: [0,4], dest: [0,2] } => println!("White castles queenside."),
            Move { target: Piece{piece: 'K', color: 1} , orig: [0,4], dest: [0,6] } => println!("White castles kingside."),
            Move { target: Piece{piece: 'K', color: 2} , orig: [7,4], dest: [7,2] } => println!("Black castles queenside."),
            Move { target: Piece{piece: 'K', color: 2} , orig: [7,4], dest: [7,6] } => println!("Black castles kingside."),
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