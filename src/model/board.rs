//* Misc public functions for board. */
use crate::model::structs::{Board, Piece, Square, Move};
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
        castle: [[false,false], [true, true], [true, true]], 
        color: 1
        }
    }

    pub fn get_piece_selection(&self, sq: Square) {
        println!("{} selected.", self.board[sq[0]][sq[1]].to_string())
    }

    pub fn get_king_square(&self, colour: usize) -> Square {
        for i in 0..8 {
            for j in 0..8 {
                if self.board[i][j] == (Piece { piece: 'K', color: colour }) {
                    return [i,j]
                }
            }
        }
        panic!("No king found!")
    }

    pub fn log(&self) {
        println!("-------------------------");
        self.last_move.log();
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