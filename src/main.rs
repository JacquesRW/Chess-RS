mod model;
mod controller;
use crate::model::defs::*;
use crate::controller::*;
use crate::model::pieces::*;


// THIS USES [COLUMN, ROW] CONVENTION JOHN
// I BETTER NOT BE SEEING NO DUMBASS STUFF

#[cfg(test)]
mod puzzles;

pub const _START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const _TEST_POSITIONS: [&str;1] = ["r2qk2r/ppp4p/3p1pn1/3Pn1p1/2B1P3/2N2P1P/PP2QP2/R4RK1 w kq - 1 15"];

//const SHANNON_NUMS: [u64;6] = [20,400,8902,197281,4865609,119060324];

fn move_counter() {
    let mut game = Board::_new();
    //let mut game = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");
    //game.make_move(Move {target: WHITE | PAWN, orig: [1,4], dest: [3,4]});
    let positions = game.perft(6);
    println!("{} moves.", positions); 
}

fn main() {
    move_counter();
    let mut a = String::new();
    let end = std::io::stdin().read_line(&mut a).unwrap();
    println!("{}", end);
    _p_v_e(_START_POSITION, WHITE);
}