//* Implementation of methods for the Move struct. */
use crate::model::defs::{Move, ScoredMove, Piece};
use crate::model::helper::*;
use crate::model::pieces::*;

// MVV_LVA[victim][attacker]
pub const MVV_LVA: [[u8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],       // victim EMPTY
    [0, 15, 14, 13, 12, 11, 10], // victim PAWN
    [0, 25, 24, 23, 22, 21, 20], // victim KNIGHT
    [0, 35, 34, 33, 32, 31, 30], // victim BISHOP
    [0, 45, 44, 43, 42, 41, 40], // victim ROOK
    [0, 55, 54, 53, 52, 51, 50], // victim QUEEN
    [0, 0, 0, 0, 0, 0, 0],       // victim KING
];


impl Move {
    #[inline(always)]
    pub fn score(&self, victim: Piece) -> u8 {
        MVV_LVA[name(victim) as usize][name(self.target) as usize]
    }
    
    pub fn null() -> Move {
        Move {target: EMPTY, orig: [0,0], dest: [0,0]}
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
