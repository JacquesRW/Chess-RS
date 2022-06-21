pub mod types;
pub mod structs;
pub mod make_move;
pub mod validation;
pub mod helper;
use structs::{Piece, Move, Board};

impl Board {
    pub fn new() -> Board {
        Board {
        board: [[Piece::rook(1),Piece::knight(1),Piece::bishop(1),Piece::queen(1),Piece::king(1),Piece::bishop(1),Piece::knight(1),Piece::rook(1)],
        [Piece::pawn(1),Piece::pawn(1),Piece::pawn(1),Piece::pawn(1),Piece::pawn(1),Piece::pawn(1),Piece::pawn(1),Piece::pawn(1)],
        [Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty()],
        [Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty()],
        [Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty()],
        [Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty()],
        [Piece::pawn(2),Piece::pawn(2),Piece::pawn(2),Piece::pawn(2),Piece::pawn(2),Piece::pawn(2),Piece::pawn(2),Piece::pawn(2)],
        [Piece::rook(2),Piece::knight(2),Piece::bishop(2),Piece::queen(2),Piece::king(2),Piece::bishop(2),Piece::knight(2),Piece::rook(2)]], 
        
        last_move: Move::null(), 
        //  castle = [
        //  [has p1 castled, has p2 castled],
        //  [can p1 castle left, can p1 castle right],
        //  [can p2 castle left, can p2 castle right]]
        castle: [[false,false], [true, true], [true, true]], 
        color: 1
        }
    }

    pub fn log(&self) {
        println!("-------------------------");
        let board_ref: &[[Piece;8];8] = &self.board;
        for i in 0..8 {
            let mut line: String = String::from(" ");
            for j in 0..8 {
                line.push_str(&(board_ref[7-i][j].repr()));
                line.push_str(" ");
            }
            println!("{}", line)
        }
        println!("-------------------------");
    }
}
