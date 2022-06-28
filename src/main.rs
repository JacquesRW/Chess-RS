mod model;
mod controller;
use crate::controller::*;
use crate::model::{pieces::*,defs::*};


// THIS USES [COLUMN, ROW] CONVENTION JOHN
// I BETTER NOT BE SEEING NO DUMBASS STUFF

#[cfg(test)]
mod puzzles;

pub const _START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const _TEST_POSITIONS: [&str;1] = ["r2qk2r/ppp4p/3p1pn1/3Pn1p1/2B1P3/2N2P1P/PP2QP2/R4RK1 w kq - 1 15"];


fn main() {
    let mut game = Board::_new();  
    //game.make_move( Move { target: WHITE | PAWN, orig: [1,1], dest: [3,1]});
    //game.log();
    //game.make_move( Move { target: BLACK | PAWN, orig: [6,0], dest: [5,0]});
    //game.log();
    //game.make_move( Move { target: WHITE | PAWN, orig: [3,1], dest: [4,1]});
    //game.log();
    //game.make_move( Move { target: BLACK | PAWN, orig: [6,2], dest: [4,2]});
    //game.log();
    //game.make_move( Move { target: WHITE | PAWN, orig: [4,1], dest: [5,1]});
    //game.log();
    let positions = game._root_perft(6);
    println!("{positions}");
    _p_v_e(_START_POSITION, WHITE);
}

// black pawn, C5 to C4
