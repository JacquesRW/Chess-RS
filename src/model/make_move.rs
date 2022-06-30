//* Makes moves on board. */

use crate::model::defs::*;
use crate::model::engine::eval::*;
use crate::model::pieces::*;

impl Board {
    #[inline(always)]
    pub fn switch_color(&mut self) {
        // switches the color to move
        self.color = other_colour(self.color);
    }

    #[inline(always)]
    fn try_en_passant(&mut self, m: &Move) {
        // assumes en passant has been validated and just removes the taken piece
        if (m.target == WHITE | PAWN) && (m.orig[0] == 4) && (self.last_move == (Move {target: BLACK | PAWN, orig: [6,m.dest[1]], dest: [4,m.dest[1]]})) {
            self.board[4][m.dest[1]] = EMPTY;
        }
        else if (m.target == BLACK | PAWN) && (m.orig[0] == 3) && (self.last_move == (Move {target: WHITE | PAWN, orig: [1,m.dest[1]], dest: [3,m.dest[1]]}))  {
            self.board[3][m.dest[1]] = EMPTY;
        }
    }
    
    #[inline(always)]
    fn try_castle(&mut self, &m: &Move) { 
        // assumes castling has been validated and looks for the specific 4 moves that indicate castling
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
        // if king moves, can't castle at all
        if m.target == WHITE | KING {self.castle &= !(WHITE_QS | WHITE_KS)}
        if m.target == BLACK | KING {self.castle &= !(BLACK_QS | BLACK_KS)}
        // if rook moves, or is taken, can't castle to its respective side
        else if (m.orig == [0,0]) || (m.dest == [0,0]) {self.castle &= !WHITE_QS}
        else if (m.orig == [0,7]) || (m.dest == [0,7]) {self.castle &= !WHITE_KS}
        else if (m.orig == [7,0]) || (m.dest == [7,0]) {self.castle &= !BLACK_QS}
        else if (m.orig == [7,7]) || (m.dest == [7,7]) {self.castle &= !BLACK_KS}
    }

    #[inline(always)]
    pub fn check_for_mate(&mut self) -> Option<bool> {
        // finds all possible moves - slow and not utilised by anything else
        // equivalent code is usually inlined where its needed but uses the found moves
        // rather than regenerating them
        let possible_moves = self.find_all_possible_moves();
        if possible_moves.is_empty() {
            if self.check_for_check_static(self.kings[colour_to_index(self.color)], self.color) {
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
        // only only promotion to queens at the moment
        let colo = colour(self.color);
        self.board[m.dest[0]][m.dest[1]] = colo | QUEEN;
    }

    #[inline(always)]
    pub fn pseudo_move(&mut self, m: Move) {
        // updating self.kings if relevant
        if m.target == WHITE | KING { self.kings[0] = m.dest }
        else if m.target == BLACK | KING { self.kings[1] = m.dest }
        // updates self.phase
        self.phase -= phase_value(self.board[m.dest[0]][m.dest[1]]);
        // destination receives the target piece
        self.board[m.dest[0]][m.dest[1]] = m.target;
        // target's old square is emptied
        self.board[m.orig[0]][m.orig[1]] = EMPTY;
        // en passant is applied to take piece if necessary
        self.try_en_passant(&m);
        // promotion is triedif relevant
        if (m.dest[0] == 7 || m.dest[0] == 0) && name(m.target) == PAWN {
            self.try_promote(&m);
        }
        // castling is looked at if it is theoretically possible
        if self.castle != NO_RIGHTS {
            self.try_castle(&m);
            self.update_castle(&m);
        }
        // last move and colour to move updated
        self.last_move = m;
        self.switch_color();
    }

    #[inline(always)]
    pub fn make_move(&mut self, m: Move) -> Option<bool> {
        // checks for mate and returns Some(true) (if mate), Some(false) (if stalemate), None (else)
        // should only be used when actually playing moves as adds significant overhead
        self.pseudo_move(m);
        self.check_for_mate()
    }
}