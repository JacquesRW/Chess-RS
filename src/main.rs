#![allow(dead_code)]
#![allow(unused_imports)]
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

pub const _TEST_POSITIONS: [&str;2] = ["r2qk2r/ppp4p/3p1pn1/3Pn1p1/2B1P3/2N2P1P/PP2QP2/R4RK1 w kq - 1 15",
                                        "r2qk2r/pPp4p/3p1pn1/3Pn1p1/2B1P3/2N2P1P/PP2QP2/R4RK1 w kq - 1 15"];

use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

fn main() {
    p_v_p(_TEST_POSITIONS[1]);
}