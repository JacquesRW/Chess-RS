//* Unmakes moves on board. */
use crate::model::defs::{Board, Move, Piece};
use crate::model::pieces::*;

impl Board {
    #[inline(always)]
    fn untry_en_passant(&mut self, m: Move, pen_move: Move) {
        if pen_move == (Move {target: BLACK | PAWN, orig: [6, m.dest[1]], dest: [4, m.dest[1]]}) {
            self.board[4][m.dest[1]] = BLACK | PAWN;
        }
        else if pen_move == (Move {target: WHITE | PAWN, orig: [1, m.dest[1]], dest: [3, m.dest[1]]})  {
            self.board[3][m.dest[1]] = WHITE | PAWN;
        }
    }

    #[inline(always)]
    fn untry_castle(&mut self, m: Move) {
        if m == (Move { target: WHITE | KING, orig: [0,4], dest: [0,2] }) {
            self.board[0][0] = WHITE | ROOK;
            self.board[0][3] = EMPTY;
        }
        else if m == (Move { target: WHITE | KING, orig: [0,4], dest: [0,6] }) {
            self.board[0][7] = WHITE | ROOK;
            self.board[0][5] = EMPTY;
        }
        else if m == (Move { target: BLACK | KING, orig: [7,4], dest: [7,2] }) {
            self.board[7][0] = BLACK | ROOK;
            self.board[7][3] = EMPTY;
        }
        else if m == (Move { target: BLACK | KING, orig: [7,4], dest: [7,6] }) {
            self.board[7][7] = BLACK | ROOK;
            self.board[7][5] = EMPTY;
        }
    }

    #[inline(always)]
    pub fn unmake_move(&mut self, m: Move, pen_move: Move, pen_castle: [[bool;2];2], capture: Piece) {
        self.board[m.orig[0]][m.orig[1]] = m.target;
        self.board[m.dest[0]][m.dest[1]] = capture;
        self.untry_en_passant(m, pen_move);
        self.untry_castle(m);
        self.last_move = pen_move;
        self.castle = pen_castle;
        self.switch_color();
    }
}