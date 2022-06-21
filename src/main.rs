mod model;
use crate::model::types::*;
use crate::model::*;

// THIS USES [COLUMN, ROW] CONVENTION JOHN
// I BETTER NOT BE SEEING NO DUMBASS STUFF

fn main() {
    let mut game = Game::new();
    game.log();
    let m = Move::new(Piece::pawn(1),[1,4],[3,4]);
    game.make_move(m);
    game.log();
}
