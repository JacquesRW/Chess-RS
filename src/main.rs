mod model;
mod controller;
use crate::controller::*;
use crate::model::pieces::*;


// THIS USES [COLUMN, ROW] CONVENTION JOHN
// I BETTER NOT BE SEEING NO DUMBASS STUFF

#[cfg(test)]
mod puzzles;

pub const _START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const _TEST_POSITIONS: [&str;1] = ["r2qk2r/ppp4p/3p1pn1/3Pn1p1/2B1P3/2N2P1P/PP2QP2/R4RK1 w kq - 1 15"];

fn main() {
    _p_v_e(_START_POSITION, WHITE);
}