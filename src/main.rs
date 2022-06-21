mod util;
use crate::util::piece::Piece;
use crate::util::moves::Move;
use crate::util::*;

fn main() {
    let mut game = Board::new();
    game.log();
    let m = Move::new(Piece::pawn(1),[1,4],[3,4]);
    game.make_move(m);
    game.log();
}
