//* Misc public functions for board. */
use crate::model::defs::{Board, Piece, Square, Move};
use crate::model::pieces::*;
impl Board {
    pub fn new() -> Board {
        Board {
        board: [[WHITE | ROOK, WHITE | KNIGHT, WHITE | BISHOP, WHITE | QUEEN, WHITE | KING, WHITE | BISHOP, WHITE | KNIGHT, WHITE | ROOK],
        [WHITE | PAWN, WHITE | PAWN, WHITE | PAWN, WHITE | PAWN, WHITE | PAWN, WHITE | PAWN, WHITE | PAWN, WHITE | PAWN],
        [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        [BLACK | PAWN, BLACK | PAWN, BLACK | PAWN, BLACK | PAWN, BLACK | PAWN, BLACK | PAWN, BLACK | PAWN, BLACK | PAWN],
        [BLACK | ROOK, BLACK | KNIGHT, BLACK | BISHOP, BLACK | QUEEN, BLACK | KING, BLACK | BISHOP, BLACK | KNIGHT, BLACK | ROOK]],  
        last_move: Move::null(), 
        castle: [[false,false], [true, true], [true, true]], 
        color: WHITE,
        // stuff for implementing unmake_move
        capture: None,
        prev_castle: [[false,false], [true, true], [true, true]],
        prev_move: Move::null(),
        best_move: Move::null()
        }
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
    }
}