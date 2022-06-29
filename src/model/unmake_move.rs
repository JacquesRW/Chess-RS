//* Unmakes moves on board. */
use crate::model::defs::{Board, Move, Piece, Castle};
use crate::model::engine::eval::*;
use crate::model::pieces::*;

impl Board {
    #[inline(always)]
    fn untry_en_passant(&mut self, m: Move, pen_move: Move) {
        // white en passant
        if pen_move == (Move {target: BLACK | PAWN, orig: [6, m.dest[1]], dest: [4, m.dest[1]]}) {
            self.board[4][m.dest[1]] = BLACK | PAWN;
        }
        // black en passant
        else if pen_move == (Move {target: WHITE | PAWN, orig: [1, m.dest[1]], dest: [3, m.dest[1]]})  {
            self.board[3][m.dest[1]] = WHITE | PAWN;
        }
    }

    #[inline(always)]
    fn untry_castle(&mut self, m: Move) {
        // white queenside
        if m == (Move { target: WHITE | KING, orig: [0,4], dest: [0,2] }) {
            self.board[0][0] = WHITE | ROOK;
            self.board[0][3] = EMPTY;
        }
        // white kingside
        else if m == (Move { target: WHITE | KING, orig: [0,4], dest: [0,6] }) {
            self.board[0][7] = WHITE | ROOK;
            self.board[0][5] = EMPTY;
        }
        // black queenside
        else if m == (Move { target: BLACK | KING, orig: [7,4], dest: [7,2] }) {
            self.board[7][0] = BLACK | ROOK;
            self.board[7][3] = EMPTY;
        }
        // black kingside
        else if m == (Move { target: BLACK | KING, orig: [7,4], dest: [7,6] }) {
            self.board[7][7] = BLACK | ROOK;
            self.board[7][5] = EMPTY;
        }
    }

    #[inline(always)]
    pub fn unmake_move(&mut self, m: Move, pen_move: Move, pen_castle: Castle, capture: Piece) {
        // restore self.phase
        self.phase += phase_value(capture);
        // return target to its original square
        self.board[m.orig[0]][m.orig[1]] = m.target;
        // restore captured (or empty) piece
        self.board[m.dest[0]][m.dest[1]] = capture;
        // add back en passant captured piece if applicable
        self.untry_en_passant(m, pen_move);
        // restore rook position if castled
        self.untry_castle(m);
        // restoring board data
        self.last_move = pen_move;
        self.castle = pen_castle;
        self.switch_color();
    }
}