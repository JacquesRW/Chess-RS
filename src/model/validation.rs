use crate::model::defs::{Piece, Board, Square, Move};
use crate::other_colour;
use std::cmp::min;

impl Board {
    fn _pawn_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
        let col = sq[0];
        let row = sq[1];
        if piece.color == 1 {
            if col == 4 {
                if row <= 6 && self.last_move == (Move { target: Piece {piece: 'P', color: 2}, orig: [row+1, 6], dest: [row+1, 4] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [5, row+1] });}
                if row >= 1 && self.last_move == (Move { target: Piece {piece: 'P', color: 2}, orig: [row-1,6], dest: [row-1, 4] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [5, row-1] });}}
            if col <= 6 && self.board[col+1][row] == (Piece {piece: 'e', color: 0}) {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col+1, row] });
                if col == 1 && self.board[col+2][row] == (Piece {piece: 'e', color: 0}) {possible_moves.push(Move { target: piece, orig: sq, dest: [col+2, row] })}}
            if row >= 1 && col <= 6 && self.board[col+1][row-1].color == 2 {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1, row-1] })}
            if row <= 6 && col <= 6 && self.board[col+1][row+1].color == 2 {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1, row+1] })}
        }
        if piece.color == 2 {
            if col == 3 {
                if row <= 6 && self.last_move == (Move { target: Piece {piece: 'P', color: 1}, orig: [row+1, 1], dest: [row+1, 3] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [2, row+1] });}
                if row >= 1 && self.last_move == (Move { target: Piece {piece: 'P', color: 1}, orig: [row-1,1], dest: [row-1, 3] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [2, row-1] });}}
            if col >= 1 && self.board[col-1][row] == (Piece {piece: 'e', color: 0}) {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col-1, row] });
                if col == 6 && self.board[col-2][row] == (Piece {piece: 'e', color: 0}) {possible_moves.push(Move { target: piece, orig: sq, dest: [col-2, row] })}}
            if row >= 1 && col >= 1 && self.board[col-1][row-1].color == 1 {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1, row-1] })}
            if row <= 6 && col >= 1 && self.board[col-1][row+1].color == 1 {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1, row+1] })}
        }
        possible_moves
    }

    fn _king_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut possible_moves: Vec<Move> = Vec::new();
        if self.castle[piece.color][0] && self.board[col][1] == (Piece {piece: 'e', color: 0}) && self.board[col][2] == (Piece {piece: 'e', color: 0}) && self.board[col][3] == (Piece {piece: 'e', color: 0}){
            possible_moves.push(Move { target: piece, orig: sq, dest: [col,2]})
        }
        if self.castle[piece.color][1] && self.board[col][5] == (Piece {piece: 'e', color: 0}) && self.board[col][6] == (Piece {piece: 'e', color: 0}) {
            possible_moves.push(Move { target: piece, orig: sq, dest: [col,6]})
        }
        if row<=6 && self.board[col][row+1].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col,row+1]})}
        if row>=1 && self.board[col][row-1].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col,row-1]})}
        if col<=6 && self.board[col+1][row].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1,row]})}
        if col>=1 && self.board[col-1][row].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1,row]})}
        if row<=6 && col<=6 && self.board[col+1][row+1].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1,row+1]})}
        if row>=1 && col>=1 && self.board[col-1][row-1].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1,row-1]})}
        if row<=6 && col>=1 && self.board[col-1][row+1].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1,row+1]})}
        if row>=1 && col<=6 && self.board[col+1][row-1].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1,row-1]})}
        possible_moves
     }
    fn _rook_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut possible_moves: Vec<Move> = Vec::new();
        for drow in 1..(row+1) {
            if row<drow { break }
            if self.board[col][row - drow].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col,row-drow]});
                if self.board[col][row - drow].color != 0 { break }
            }
            else { break }
        }
        for drow in 1..(8-row) {
            if row+drow>=8 { break }
            if self.board[col][row + drow].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col,row+drow]});
                if self.board[col][row + drow].color != 0 { break }
            }
            else { break }
        }
        for dcol in 1..(col+1) {
            if col<dcol { break }
            if self.board[col-dcol][row].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col-dcol,row]});
                if self.board[col-dcol][row].color != 0 { break }
            }
            else { break }
        }
        for dcol in 1..(8-row) {
            if col+dcol>=8 { break }
            if self.board[col+dcol][row].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col+dcol,row]});
                if self.board[col+dcol][row].color != 0 { break }
            }
            else { break }
        }
        possible_moves
    }
    fn _bishop_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut smaller = min(col, row);
        let mut possible_moves: Vec<Move> = Vec::new();
        for change in 1..(smaller+1) {
            if self.board[col-change][row-change].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col-change,row-change]});
                if self.board[col-change][row-change].color != 0 { break }
            }
            else { break }
        }
        smaller = min(7-col, row);
        for change in 1..(smaller+1) {
            if self.board[col+change][row-change].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col+change,row-change]});
                if self.board[col+change][row-change].color != 0 { break }
            }
            else { break }
        }
        smaller = min(col, 7-row);
        for change in 1..(smaller+1) {
            if self.board[col-change][row+change].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col-change,row+change]});
                if self.board[col-change][row+change].color != 0 { break }
            }
            else { break }
        }
        smaller = min(7-col, 7-row);
        for change in 1..(smaller+1) {
            if self.board[col+change][row+change].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col+change,row+change]});
                if self.board[col+change][row+change].color != 0 { break }
            }
            else { break }
        }
        possible_moves
    }
    fn _knight_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut possible_moves: Vec<Move> = Vec::new();
        if row<=5 && col<=6 && self.board[col+1][row+2].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1,row+2] })}
        if row>=2 && col<=6 && self.board[col+1][row-2].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1,row-2] })}
        if row<=5 && col>=1 && self.board[col-1][row+2].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1,row+2] })}
        if row>=2 && col>=1 && self.board[col-1][row-2].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1,row-2] })}
        if row<=6 && col<=5 && self.board[col+2][row+1].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col+2,row+1] })}
        if row>=1 && col<=5 && self.board[col+2][row-1].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col+2,row-1] })}
        if row<=6 && col>=2 && self.board[col-2][row+1].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col-2,row+1] })}
        if row>=1 && col>=2 && self.board[col-2][row-1].color != piece.color {possible_moves.push(Move { target: piece, orig: sq, dest: [col-2,row-1] })}
        possible_moves
    }
    
    fn _queen_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let mut possible_moves = self._rook_moves(sq, piece);
        let mut additional_moves = self._bishop_moves(sq,piece);
        possible_moves.append(&mut additional_moves);
        possible_moves
    }

    fn unvalidated_moves(&self, sq: Square) -> Vec<Move> {
        let piece = self.board[sq[0]][sq[1]];
        if piece.color != self.color {panic!("Not a valid piece")}
        match piece.piece {
            'P' => self._pawn_moves(sq, piece),
            'Q' => self._queen_moves(sq, piece),
            'R' => self._rook_moves(sq, piece),
            'N' => self._knight_moves(sq, piece),
            'B' => self._bishop_moves(sq, piece),
            'K' => self._king_moves(sq, piece),
            _ => panic!("Not a valid piece!")
        }
    }

    fn _find_all_unvalidated_moves(&self) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
        for column in 0..8 {
            for row in 0..8 {
                if self.board[column][row].color == self.color {
                    let mut current_moves = self.unvalidated_moves([column,row]);
                    possible_moves.append(&mut current_moves);
                }
            }
        }
        possible_moves
    }

    fn check_for_check(&self, m: Move, king_square: Square, colour: usize) -> bool {
        let mut temp = self.clone();
        temp.make_move(m);
        temp.color = colour;
        let alt_color = other_colour(temp.color);
        for pos in temp._pawn_moves(king_square, Piece { piece: 'e', color: colour }) {
            if temp.board[pos.dest[0]][pos.dest[1]] == (Piece { piece: 'P', color: alt_color}) {
                return true
            }
        }
        for pos in temp._knight_moves(king_square, Piece { piece: 'e', color: colour }) {
            if temp.board[pos.dest[0]][pos.dest[1]] == (Piece { piece: 'N', color: alt_color}) {
                return true
            }
        }
        for pos in temp._rook_moves(king_square, Piece { piece: 'e', color: colour }) {
            if temp.board[pos.dest[0]][pos.dest[1]] == (Piece { piece: 'R', color: alt_color}) || temp.board[pos.dest[0]][pos.dest[1]] == (Piece { piece: 'Q', color: alt_color}) {
                return true 
            }
        }
        for pos in temp._bishop_moves(king_square, Piece { piece: 'e', color: colour }) {
            if temp.board[pos.dest[0]][pos.dest[1]] == (Piece { piece: 'B', color: alt_color}) || temp.board[pos.dest[0]][pos.dest[1]] == (Piece { piece: 'Q', color: alt_color}) {
                return true 
            }
        }
        false
    }

    fn possible_moves(&self, sq: Square, king_square: Square, colour: usize) -> Vec<Move> {
        let unvalidated = self.unvalidated_moves(sq);
        let mut possible_moves: Vec<Move> = Vec::new();
        if self.board[sq[0]][sq[1]] == (Piece { piece: 'K', color: colour}) {
            for m in unvalidated {
                if !(self.check_for_check(m, m.dest, colour)) {
                    possible_moves.push(m);
                }
            }
            return possible_moves
        }
        else {
            for m in unvalidated {
                if !(self.check_for_check(m, king_square, colour)) {
                    possible_moves.push(m);
                }
            }
            return possible_moves
        }
    }

    pub fn selection_possible_moves(&self, sq: Square) -> Vec<Move> {
        let king_square = self.get_king_square(self.color);
        self.possible_moves(sq, king_square, self.color)
    }

    pub fn find_all_possible_moves(&self) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
        let king_square = self.get_king_square(self.color);
        for column in 0..8 {
            for row in 0..8 {
                if self.board[column][row].color == self.color {
                    let mut current_moves = self.possible_moves([column,row], king_square, self.color);
                    possible_moves.append(&mut current_moves);
                }
            }
        }
        possible_moves
    }

}