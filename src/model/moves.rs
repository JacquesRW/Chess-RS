//* Implementation of methods for the Move struct. */
use crate::model::defs::{Piece, Square, Move, ScoredMove};
use crate::model::helper::*;
use crate::model::pieces::*;

impl Move {

    pub fn null() -> Move {
        Move {target: EMPTY, orig: [0,0], dest: [0,0]}
    }

    pub fn _reverse(&self) -> Move {
        Move { target: self.target, orig: self.dest, dest: self.orig}
    }
    #[inline(always)]
    pub fn new(piece: Piece, origin: Square, destination: Square) -> Move {
        Move {target: piece, orig: origin, dest: destination}
    }

    pub fn log(&self) {
        match self {
            Move { target: WHITE | KING , orig: [0,4], dest: [0,2] } => println!("White castles queenside."),
            Move { target: WHITE | KING , orig: [0,4], dest: [0,6] } => println!("White castles kingside."),
            Move { target: BLACK | KING , orig: [7,4], dest: [7,2] } => println!("Black castles queenside."),
            Move { target: BLACK | KING , orig: [7,4], dest: [7,6] } => println!("Black castles kingside."),
            _ => println!("{}, {} to {}", as_string(self.target), get_coords(&self.orig), get_coords(&self.dest))
        }
    }

    pub fn _to_string(&self) -> String {
        match self {
            Move { target: WHITE | KING , orig: [0,4], dest: [0,2] } => format!("castle queenside."),
            Move { target: WHITE | KING , orig: [0,4], dest: [0,6] } => format!("castle kingside."),
            Move { target: BLACK | KING , orig: [7,4], dest: [7,2] } => format!("castle queenside."),
            Move { target: BLACK | KING , orig: [7,4], dest: [7,6] } => format!("castle kingside."),
            _ => format!("{}, {} to {}", as_string(self.target), get_coords(&self.orig), get_coords(&self.dest))
        }
    }

    pub fn _print_destinations(moves: &Vec<Self>) {
        let mut message = String::from("Possible Destinations: ");
        for m in moves {
            message += &get_coords(&m.dest);
            message += &String::from(" ");
        }
        println!("{message}")
    }
}

pub fn _output_move_list(move_list: &Vec<ScoredMove>) {
    // MOSTLY MEANINGLESS
    // due to pruning strategies worse moves will not be effectively evaluated
    // so will only return an upper bound for them
    for m in move_list {
        println!("{} Eval: {}", m.m._to_string(), m.s)
    }
}
