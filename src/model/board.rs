//* Misc public functions for board. */
use crate::model::defs::{Board, Piece, Square};
use crate::model::pieces::*;
impl Board {
    pub fn _get_piece_selection(&self, sq: Square) {
        // logs piece selected in console
        println!("{} selected.", as_string(self.board[sq[0]][sq[1]]))
    }

    #[inline(always)]
    pub fn get_king_square(&self, colour: u8) -> Square {
        // finds the king location for the given colour
        let pc = KING | colour;
        for i in 0..8 {
            for j in 0..8 {
                if self.board[i][j] == pc {
                    return [i,j]
                }
            }
        }
        self._log();
        panic!("No king found!")
    }
    pub fn _log(&self) {
        // console output of the board, last move and side to move
        println!("---------------------------------");
        self.last_move.log();
        println!("+---+---+---+---+---+---+---+---+");
        let board_ref: &[[Piece;8];8] = &self.board;
        for i in 0..8 {
            let mut line: String = String::from("| ");
            for j in 0..8 {
                line.push_str(&repr(board_ref[7-i][j]));
                line.push_str(" | ");
            }
            println!("{}", line);
            println!("+---+---+---+---+---+---+---+---+");
        }
        println!("{} to move.", match self.color { WHITE => "White", BLACK => "Black", _ => panic!("Invalid colour!")});
        println!("---------------------------------");
    }
}