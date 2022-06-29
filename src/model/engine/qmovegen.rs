//* Generates possible moves for quiescence search. */

use crate::model::defs::{Piece, Board, Square, Move};
use crate::model::pieces::*;
use std::cmp::min;

impl Board {
    fn _pawn_takes_promotions(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let mut possible_takes: Vec<Move> = Vec::new();
        let col = sq[0];
        let row = sq[1];
        if colour(piece) == WHITE {
            // promotion
            if col == 6 && self.board[7][row] == EMPTY {
                possible_takes.push(Move { target: piece, orig: sq, dest: [7,row]})
            }
            // en passants
            if col == 4 {
                if row <= 6 && self.last_move == (Move { target: PAWN | BLACK , orig: [6, row+1], dest: [4, row+1] }) {
                    possible_takes.push(Move { target: piece, orig: sq, dest: [5, row+1] });}
                if row >= 1 && self.last_move == (Move { target: PAWN | BLACK, orig: [6, row-1], dest: [4, row-1] }) {
                    possible_takes.push(Move { target: piece, orig: sq, dest: [5, row-1] });}}
            // takes
            if row >= 1 && col <= 6 && colour(self.board[col+1][row-1]) == BLACK {possible_takes.push(Move { target: piece, orig: sq, dest: [col+1, row-1] })}
            if row <= 6 && col <= 6 && colour(self.board[col+1][row+1]) == BLACK {possible_takes.push(Move { target: piece, orig: sq, dest: [col+1, row+1] })}
        }
        if colour(piece) == BLACK {
            // promotion
            if col == 1 && self.board[0][row] == EMPTY {
                possible_takes.push(Move { target: piece, orig: sq, dest: [0,row]})
            }
            // en passants
            if col == 3 {
                if row <= 6 && self.last_move == (Move { target: WHITE | PAWN, orig: [1, row+1], dest: [3, row+1] }) {
                    possible_takes.push(Move { target: piece, orig: sq, dest: [2, row+1] });}
                if row >= 1 && self.last_move == (Move { target: WHITE | PAWN, orig: [1, row-1], dest: [3, row-1] }) {
                    possible_takes.push(Move { target: piece, orig: sq, dest: [2, row-1] });}}
            // takes
            if row >= 1 && col >= 1 && colour(self.board[col-1][row-1]) == WHITE {possible_takes.push(Move { target: piece, orig: sq, dest: [col-1, row-1] })}
            if row <= 6 && col >= 1 && colour(self.board[col-1][row+1]) == WHITE {possible_takes.push(Move { target: piece, orig: sq, dest: [col-1, row+1] })}
        }
        possible_takes
    }

    #[inline(always)]
    fn _king_takes(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let colo = other_colour(colour(piece));
        let mut possible_takes: Vec<Move> = Vec::new();
        if row<=6 && self.board[col][row+1] == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col,row+1]})}
        if row>=1 && self.board[col][row-1] == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col,row-1]})}
        if col<=6 && self.board[col+1][row] == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col+1,row]})}
        if col>=1 && self.board[col-1][row] == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col-1,row]})}
        if row<=6 && col<=6 && self.board[col+1][row+1] == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col+1,row+1]})}
        if row>=1 && col>=1 && self.board[col-1][row-1] == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col-1,row-1]})}
        if row<=6 && col>=1 && self.board[col-1][row+1] == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col-1,row+1]})}
        if row>=1 && col<=6 && self.board[col+1][row-1] == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col+1,row-1]})}
        possible_takes
    }

    fn _rook_takes(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut possible_takes: Vec<Move> = Vec::new();
        let alt_colo = other_colour(colour(piece));
        for drow in 1..(row+1) {
            if row<drow { break }
            if colour(self.board[col][row - drow]) == alt_colo {
                possible_takes.push(Move { target: piece, orig: sq, dest: [col,row-drow]});
                break;
            }
            if colour(self.board[col][row - drow]) != EMPTY {break;}
        }
        for drow in 1..(8-row) {
            if row+drow>=8 { break }  
            if colour(self.board[col][row + drow]) == alt_colo {
                possible_takes.push(Move { target: piece, orig: sq, dest: [col,row+drow]});
                break;
            }
            if colour(self.board[col][row + drow]) != EMPTY {break;}
        }
        for dcol in 1..(col+1) {
            if col<dcol { break }
            if colour(self.board[col-dcol][row]) == alt_colo {
                possible_takes.push(Move { target: piece, orig: sq, dest: [col-dcol,row]});
                break;
            }
            if colour(self.board[col-dcol][row]) != EMPTY {break;}
        }
        for dcol in 1..(8-col) {
            if col+dcol>=8 { break }
            if colour(self.board[col+dcol][row]) == alt_colo {
                possible_takes.push(Move { target: piece, orig: sq, dest: [col+dcol,row]});
                break;
            }
            if colour(self.board[col+dcol][row]) != EMPTY {break;}
        }
        possible_takes
    }

    fn _bishop_takes(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut smaller = min(col, row);
        let mut possible_takes: Vec<Move> = Vec::new();
        let alt_colo = other_colour(colour(piece));
        for change in 1..(smaller+1) {
            if colour(self.board[col-change][row-change]) == alt_colo {
                possible_takes.push(Move { target: piece, orig: sq, dest: [col-change,row-change]});
            }
            if colour(self.board[col-change][row-change]) != EMPTY { break }
        }
        smaller = min(7-col, row);
        for change in 1..(smaller+1) {
            if colour(self.board[col+change][row-change]) == alt_colo {
                possible_takes.push(Move { target: piece, orig: sq, dest: [col+change,row-change]});
            }
            if colour(self.board[col+change][row-change]) != EMPTY { break }
        }
        smaller = min(col, 7-row);
        for change in 1..(smaller+1) {
            if colour(self.board[col-change][row+change]) == alt_colo {
                possible_takes.push(Move { target: piece, orig: sq, dest: [col-change,row+change]});
            }
            if colour(self.board[col-change][row+change]) != EMPTY { break }
        }
        smaller = min(7-col, 7-row);
        for change in 1..(smaller+1) {
            if colour(self.board[col+change][row+change]) == alt_colo {
                possible_takes.push(Move { target: piece, orig: sq, dest: [col+change,row+change]});
            }
            if colour(self.board[col+change][row+change]) != EMPTY { break }
        }
        possible_takes
    }

    fn _knight_takes(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut possible_takes: Vec<Move> = Vec::new();
        let colo = other_colour(colour(piece));
        if row<=5 && col<=6 && colour(self.board[col+1][row+2]) == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col+1,row+2] })}
        if row>=2 && col<=6 && colour(self.board[col+1][row-2]) == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col+1,row-2] })}
        if row<=5 && col>=1 && colour(self.board[col-1][row+2]) == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col-1,row+2] })}
        if row>=2 && col>=1 && colour(self.board[col-1][row-2]) == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col-1,row-2] })}
        if row<=6 && col<=5 && colour(self.board[col+2][row+1]) == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col+2,row+1] })}
        if row>=1 && col<=5 && colour(self.board[col+2][row-1]) == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col+2,row-1] })}
        if row<=6 && col>=2 && colour(self.board[col-2][row+1]) == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col-2,row+1] })}
        if row>=1 && col>=2 && colour(self.board[col-2][row-1]) == colo {possible_takes.push(Move { target: piece, orig: sq, dest: [col-2,row-1] })}
        possible_takes
    }
    #[inline(always)]
    fn _queen_takes(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let mut possible_takes = self._rook_takes(sq, piece);
        let mut additional_takes = self._bishop_takes(sq,piece);
        possible_takes.append(&mut additional_takes);
        possible_takes
    }

    fn unvalidated_takes(&self, sq: Square) -> Vec<Move> {
        let piece = self.board[sq[0]][sq[1]];
        if colour(piece) != self.color {panic!("Not a valid piece")}
        match name(piece) {
            PAWN => self._pawn_takes_promotions(sq, piece),
            QUEEN => self._queen_takes(sq, piece),
            ROOK => self._rook_takes(sq, piece),
            KNIGHT => self._knight_takes(sq, piece),
            BISHOP => self._bishop_takes(sq, piece),
            KING => self._king_takes(sq, piece),
            _ => panic!("Not a valid piece!")
        }
    }

    #[inline(always)]
    fn possible_takes(&mut self, sq: Square, king_square: Square, colour: u8) -> Vec<Move> {
        let unvalidated = self.unvalidated_takes(sq);
        let mut possible_takes: Vec<Move> = Vec::new();
        if self.board[sq[0]][sq[1]] == KING | colour {
            for m in unvalidated {
                if !(self.check_for_check(m, m.dest, colour)) {
                    possible_takes.push(m);
                }
            }
            return possible_takes
        }
        else {
            for m in unvalidated {
                if !(self.check_for_check(m, king_square, colour)) {
                    possible_takes.push(m);
                }
            }
            return possible_takes
        }
    }

    #[inline(always)]
    pub fn find_all_possible_quiet_moves(&mut self) -> Vec<Move> {
        let mut possible_takes: Vec<Move> = Vec::new();
        let king_square = self.get_king_square(self.color);
        for column in 0..8 {
            for row in 0..8 {
                if colour(self.board[column][row]) == self.color {
                    let mut current_takes = self.possible_takes([column,row], king_square, self.color);
                    possible_takes.append(&mut current_takes);
                }
            }
        }
        possible_takes
    }
}