//* Misc public functions for board. */
use crate::model::defs::{Board, Piece, Square};
use crate::model::pieces::*;
impl Board {
    pub fn new() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn get_piece_selection(&self, sq: Square) {
        println!("{} selected.", as_string(self.board[sq[0]][sq[1]]))
    }

    pub fn get_king_square(&self, colour: u8) -> Square {
        let pc = KING | colour;
        for i in 0..8 {
            for j in 0..8 {
                if self.board[i][j] == pc {
                    return [i,j]
                }
            }
        }
        self.log();
        panic!("No king found!")
    }

    pub fn _raw_log(&self) {
        println!("-------------------------");
        self.last_move.log();
        println!("-------------------------");
        let board_ref: &[[Piece;8];8] = &self.board;
        for i in 0..8 {
            let mut line: String = String::from(" ");
            for j in 0..8 {
                line.push_str(&board_ref[7-i][j].to_string());
                line.push_str(" ");
            }
            println!("{}", line)
        }
        println!("-------------------------");
    }

    pub fn log(&self) {
        println!("-------------------------");
        self.last_move.log();
        println!("-------------------------");
        let board_ref: &[[Piece;8];8] = &self.board;
        for i in 0..8 {
            let mut line: String = String::from(" ");
            for j in 0..8 {
                line.push_str(&repr(board_ref[7-i][j]));
                line.push_str(" ");
            }
            println!("{}", line)
        }
        println!("-------------------------");
        println!("{} to move.", match self.color { WHITE => "White", BLACK => "Black", _ => panic!("Invalid colour!")});
        println!("-------------------------");
    }
}