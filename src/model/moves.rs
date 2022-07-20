//* Implementation of methods for the Move struct. */
use crate::model::defs::{Move, Piece};
use crate::model::helper::*;
use crate::model::pieces::*;

// Most-Valuable-Victim Least-Valuable-Attacker
pub const MVV_LVA: [[u8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],       // victim EMPTY
    [0, 15, 14, 13, 12, 11, 10], // victim PAWN
    [0, 25, 24, 23, 22, 21, 20], // victim KNIGHT
    [0, 35, 34, 33, 32, 31, 30], // victim BISHOP
    [0, 45, 44, 43, 42, 41, 40], // victim ROOK
    [0, 55, 54, 53, 52, 51, 50], // victim QUEEN
    [0, 0, 0, 0, 0, 0, 0],       // victim KING (should not be referenced)
];


impl Move {
    #[inline(always)]
    pub fn score(&self, victim: Piece) -> u8 {
        // scores a move for it to be ordered in engine search
        // huge performance benefit
        MVV_LVA[name(victim) as usize][name(self.target) as usize]
    }
    
    pub fn null() -> Move {
        // null move
        Move {target: EMPTY, orig: [0,0], dest: [0,0]}
    }

    pub fn _to_string(&self) -> String {
        // string of move
        match self {
            Move { target: WHITE | KING , orig: [0,4], dest: [0,2] } => format!("White castles queenside."),
            Move { target: WHITE | KING , orig: [0,4], dest: [0,6] } => format!("White castles kingside."),
            Move { target: BLACK | KING , orig: [7,4], dest: [7,2] } => format!("Black castles queenside."),
            Move { target: BLACK | KING , orig: [7,4], dest: [7,6] } => format!("Black castles kingside."),
            _ => format!("{}, {} to {}", as_string(self.target), get_coords(&self.orig), get_coords(&self.dest))
        }
    }

    pub fn log(&self) {
        // logs move in console
        println!("{}", self._to_string())
    }

    pub fn _to_uci_string(&self) -> String {
        // UCI format
        match self {
            Move { target: WHITE | KING , orig: [0,4], dest: [0,2] } => format!("castle queenside."),
            Move { target: WHITE | KING , orig: [0,4], dest: [0,6] } => format!("castle kingside."),
            Move { target: BLACK | KING , orig: [7,4], dest: [7,2] } => format!("castle queenside."),
            Move { target: BLACK | KING , orig: [7,4], dest: [7,6] } => format!("castle kingside."),
            _ => format!("{}{}", get_coords(&self.orig), get_coords(&self.dest))
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
