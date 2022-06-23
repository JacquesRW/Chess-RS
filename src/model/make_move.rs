//* Makes moves on board. */
//* Only file that should be mutating the fields of a board. */

use crate::model::defs::{Board, Move};
use crate::model::pieces::*;

impl Board {
    pub fn switch_color(&mut self) {
        self.color = other_colour(self.color);
    }
    
    fn try_en_passant(&mut self, &m: &Move) {
        if m.target == WHITE | PAWN && m.orig[0] == 4 && self.last_move == (Move {target: BLACK | PAWN, orig: [6,m.dest[1]], dest: [4,m.dest[1]]}) {
            println!("White is en passanting!");
            self.board[4][m.dest[1]] = EMPTY;
        }
        else if m.target == BLACK | PAWN && m.orig[0] == 3 && self.last_move == (Move {target: WHITE | PAWN, orig: [1,m.dest[1]], dest: [3,m.dest[1]]})  {
            println!("Black is en passanting!");
            self.board[3][m.dest[1]] = EMPTY;
        }
    }
    
    fn try_castle(&mut self) { 
        if self.last_move == (Move { target: WHITE | KING, orig: [0,4], dest: [0,2] }) {
            self.board[0][3] = WHITE | ROOK;
            self.board[0][0] = EMPTY;
            self.castle[0][0] = true;
        }
        else if self.last_move == (Move { target: WHITE | KING, orig: [0,4], dest: [0,6] }) {
            self.board[0][5] = WHITE | ROOK;
            self.board[0][7] = EMPTY;
            self.castle[0][0] = true;
        }
        else if self.last_move == (Move { target: BLACK | KING, orig: [7,4], dest: [7,2] }) {
            self.board[7][3] = BLACK | ROOK;
            self.board[7][0] = EMPTY;
            self.castle[0][1] = true;
        }
        else if self.last_move == (Move { target: BLACK | KING, orig: [7,4], dest: [7,6] }) {
            self.board[7][5] = BLACK | ROOK;
            self.board[7][7] = EMPTY;
            self.castle[0][1] = true;
        }
    }
    
    fn update_castle(&mut self) {
        let temp_colour = colour(self.last_move.target);
        if name(self.last_move.target) == KING {self.castle[(temp_colour >> 3) as usize] = [false, false]}
        else if self.last_move.orig == [0,0] || self.last_move.dest == [0,0] {self.castle[1][0] = false}
        else if self.last_move.orig == [0,7] || self.last_move.dest == [0,7] {self.castle[1][1] = false}
        else if self.last_move.orig == [7,0] || self.last_move.dest == [7,0] {self.castle[2][0] = false}
        else if self.last_move.orig == [7,7] || self.last_move.dest == [7,7] {self.castle[2][1] = false}
    }

    pub fn make_move(&mut self, m: Move) {
        if colour(m.target) != self.color {
            panic!("Wrong colour trying to move!")
        }
        if m.target != self.board[m.orig[0]][m.orig[1]] {
            panic!("Invalid move type!")
        }
        self.try_en_passant(&m);
        self.board[m.dest[0]][m.dest[1]] = m.target;
        self.board[m.orig[0]][m.orig[1]] = EMPTY;
        self.last_move = m;
        if self.castle[(self.color >> 3) as usize] != [false,false] {
            self.try_castle();
            self.update_castle();
        }
        self.switch_color();
    }

    pub fn pseudo_move(&mut self, m: Move) {
        self.try_en_passant(&m);
        self.board[m.dest[0]][m.dest[1]] = m.target;
        self.board[m.orig[0]][m.orig[1]] = EMPTY;
        self.last_move = m;
        if self.castle[(self.color >> 3) as usize] != [false,false] {
            self.try_castle();
            self.update_castle();
        self.switch_color();
        }
    }
}