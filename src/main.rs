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
mod tests;
mod puzzles;

pub const _TEST_POSITIONS: [&str;1] = ["r2qk2r/ppp4p/3p1pn1/3Pn1p1/2B1P3/2N2P1P/PP2QP2/R4RK1 w kq - 1 15"];

use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

fn main() {
    p_v_e(_TEST_POSITIONS[0], WHITE);
}