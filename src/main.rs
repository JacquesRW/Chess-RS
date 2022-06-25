mod model;
use crate::model::gamemodes::*;
use crate::model::pieces::*;
// THIS USES [COLUMN, ROW] CONVENTION JOHN
// I BETTER NOT BE SEEING NO DUMBASS STUFF

#[cfg(test)]
mod tests;
mod puzzles;

pub const TEST_POSITIONS: [&str;1] = ["r2qk2r/ppp4p/3p1pn1/3Pn1p1/2B1P3/2N2P1P/PP2QP2/R4RK1 w kq - 1 15"];

fn main() {
    p_v_e(TEST_POSITIONS[0], WHITE)
}
