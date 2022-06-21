use crate::model::structs::{Piece, Board, Move};
use crate::model::helper::*;

impl Board {
    fn switch_color(&mut self) {
        self.color = other_colour(self.color);
    }
    
    fn try_en_passant(&mut self, &m: &Move) {
        if m.target == (Piece {piece: 'P', color: 1}) {
            if m.orig[0] == 4 && (m.dest[1] == m.orig[1] - 1 || m.dest[1] == m.orig[1] + 1) && self.board[m.dest[0]][m.dest[1]] == Piece::empty() {
                self.board[m.orig[0]][m.dest[1]] = Piece::empty()
            }
        }
        if m.target == (Piece {piece: 'P', color: 2}) {
            if m.orig[0] == 3 && (m.dest[1] == m.orig[1] - 1 || m.dest[1] == m.orig[1] + 1) && self.board[m.dest[0]][m.dest[1]] == Piece::empty() {
                self.board[m.orig[0]][m.dest[1]] = Piece::empty()
            }
        }
    }
    
    fn try_castle(&mut self, &m: &Move) { 
        if m == (Move { target: Piece{piece: 'K', color: 1} , orig: [0,4], dest: [0,2] }) {
            println!("White castles Queenside.");
            self.board[0][3] = Piece::rook(1);
            self.board[0][0] = Piece::empty();
            self.castle[0][0] = true;
        }
        if m == (Move { target: Piece{piece: 'K', color: 1} , orig: [0,4], dest: [0,6] }) {
            println!("White castles Queenside.");
            self.board[0][5] = Piece::rook(1);
            self.board[0][7] = Piece::empty();
            self.castle[0][0] = true;
        }
        if m == (Move { target: Piece{piece: 'K', color: 2} , orig: [7,4], dest: [7,2] }) {
            println!("White castles Queenside.");
            self.board[7][3] = Piece::rook(2);
            self.board[7][0] = Piece::empty();
            self.castle[0][1] = true;
        }
        if m == (Move { target: Piece{piece: 'K', color: 2} , orig: [7,4], dest: [7,6] }) {
            println!("White castles Queenside.");
            self.board[7][5] = Piece::rook(2);
            self.board[7][7] = Piece::empty();
            self.castle[0][1] = true;
        }
    }
    
    fn update_castle(&mut self, m: &Move) {
        if m.target.piece == 'K' {
            self.castle[m.target.color] = [false, false]
        }
        if m.target == Piece::rook(1) {
            if m.orig == [0,0] {self.castle[m.target.color][0] = false}
            if m.orig == [0,7] {self.castle[m.target.color][1] = false}
        }
        if m.target == Piece::rook(2) {
            if m.orig == [7,0] {self.castle[m.target.color][0] = false}
            if m.orig == [7,7] {self.castle[m.target.color][1] = false}
        }
    }

    pub fn make_move(&mut self, m: Move) {
        if m.target.color != self.color {
            panic!("Wrong colour trying to move!")
        }
        if m.target != self.board[m.orig[0]][m.orig[1]] {
            panic!("Invalid move type!")
        }
        self.board[m.dest[0]][m.dest[1]] = m.target;
        self.board[m.orig[0]][m.orig[1]] = Piece::empty();
        self.try_en_passant(&m);
        self.try_castle(&m);
        self.update_castle(&m);
        self.last_move = m;
        self.switch_color();
    }
}