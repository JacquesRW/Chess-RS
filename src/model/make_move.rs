//* Makes moves on board. */
//* Only file that should be mutating the fields of a board. */

use crate::model::defs::{Board, Move};
use crate::model::pieces::*;

impl Board {
    #[inline(always)]
    pub fn switch_color(&mut self) {
        self.color = other_colour(self.color);
    }

    #[inline(always)]
    fn try_en_passant(&mut self, m: &Move) {
        if m.target == WHITE | PAWN && m.orig[0] == 4 && self.last_move == (Move {target: BLACK | PAWN, orig: [6,m.dest[1]], dest: [4,m.dest[1]]}) {
            self.board[4][m.dest[1]] = EMPTY;
        }
        else if m.target == BLACK | PAWN && m.orig[0] == 3 && self.last_move == (Move {target: WHITE | PAWN, orig: [1,m.dest[1]], dest: [3,m.dest[1]]})  {
            self.board[3][m.dest[1]] = EMPTY;
        }
    }
    
    #[inline(always)]
    fn try_castle(&mut self, &m: &Move) { 
        if m == (Move { target: WHITE | KING, orig: [0,4], dest: [0,2] }) {
            self.board[0][3] = WHITE | ROOK;
            self.board[0][0] = EMPTY;
        }
        else if m == (Move { target: WHITE | KING, orig: [0,4], dest: [0,6] }) {
            self.board[0][5] = WHITE | ROOK;
            self.board[0][7] = EMPTY;
        }
        else if m == (Move { target: BLACK | KING, orig: [7,4], dest: [7,2] }) {
            self.board[7][3] = BLACK | ROOK;
            self.board[7][0] = EMPTY;
        }
        else if m == (Move { target: BLACK | KING, orig: [7,4], dest: [7,6] }) {
            self.board[7][5] = BLACK | ROOK;
            self.board[7][7] = EMPTY;
        }
    }
    
    #[inline(always)]
    fn update_castle(&mut self, m: &Move) {
        if name(m.target) == KING {self.castle[colour_to_index(colour(m.target))] = [false, false]}
        else if m.orig == [0,0] || m.dest == [0,0] {self.castle[0][0] = false}
        else if m.orig == [0,7] || m.dest == [0,7] {self.castle[0][1] = false}
        else if m.orig == [7,0] || m.dest == [7,0] {self.castle[1][0] = false}
        else if m.orig == [7,7] || m.dest == [7,7] {self.castle[1][1] = false}
    }

    #[inline(always)]
    fn check_for_mate(&self) -> Option<bool> {
        let possible_moves = self.find_all_possible_moves();
        if possible_moves.is_empty() {
            if self.check_for_check_static(self.get_king_square(self.color), self.color) {
                return Some(true)
            }
            return Some(false)
        }
        else {
            None
        }
    }

    #[inline(always)]
    fn try_promote(&mut self, m: &Move) {
        let colo = colour(self.color);
        self.board[m.dest[0]][m.dest[1]] = colo | QUEEN;
    }

    #[inline(always)]
    pub fn pseudo_move(&mut self, m: Move) {
        // no checking for checkmate
        self.capture = self.board[m.dest[0]][m.dest[1]];
        self.try_en_passant(&m);
        self.board[m.dest[0]][m.dest[1]] = m.target;
        self.board[m.orig[0]][m.orig[1]] = EMPTY;
        if (m.dest[0] == 7 || m.dest[0] == 0) && name(m.target) == PAWN {
            self.try_promote(&m);
        }
        if self.castle[colour_to_index(self.color)] != [false,false] {
            self.try_castle(&m);
            self.update_castle(&m);
        }
        self.last_move = m;
        self.switch_color();
    }

    #[inline(always)]
    pub fn make_move(&mut self, m: Move) -> Option<bool> {
        // checks for mate and returns True (if mate), False (if stalemate), None (else)
        self.pseudo_move(m);
        self.check_for_mate()
    }
}