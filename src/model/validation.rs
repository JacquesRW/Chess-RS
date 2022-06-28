use crate::model::defs::*;
use crate::model::pieces::*;
use std::cmp::min;

impl Board {
    fn _pawn_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
        let col = sq[0];
        let row = sq[1];
        if colour(piece) == WHITE {
            // en passants
            if col == 4 {
                if row <= 6 && self.last_move == (Move { target: PAWN | BLACK , orig: [6, row+1], dest: [4, row+1] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [5, row+1] });}
                if row >= 1 && self.last_move == (Move { target: PAWN | BLACK, orig: [6, row-1], dest: [4, row-1] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [5, row-1] });}}
            // normal 1 move forwards
            if col <= 6 && self.board[col+1][row] == EMPTY {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col+1, row] });
                if col == 1 && self.board[col+2][row] == EMPTY {possible_moves.push(Move { target: piece, orig: sq, dest: [col+2, row] })}}
            // takes
            if row >= 1 && col <= 6 && colour(self.board[col+1][row-1]) == BLACK {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1, row-1] })}
            if row <= 6 && col <= 6 && colour(self.board[col+1][row+1]) == BLACK {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1, row+1] })}
        }
        if colour(piece) == BLACK {
            // en passants
            if col == 3 {
                if row <= 6 && self.last_move == (Move { target: WHITE | PAWN, orig: [1, row+1], dest: [3, row+1] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [2, row+1] });}
                if row >= 1 && self.last_move == (Move { target: WHITE | PAWN, orig: [1, row-1], dest: [3, row-1] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [2, row-1] });}}
            // normal 1 move forwards
            if col >= 1 && self.board[col-1][row] == EMPTY {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col-1, row] });
                // 2 move initial push
                if col == 6 && self.board[col-2][row] == EMPTY {possible_moves.push(Move { target: piece, orig: sq, dest: [col-2, row] })}}
            // takes
            if row >= 1 && col >= 1 && colour(self.board[col-1][row-1]) == WHITE {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1, row-1] })}
            if row <= 6 && col >= 1 && colour(self.board[col-1][row+1]) == WHITE {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1, row+1] })}
        }
        possible_moves
    }

    #[inline(always)]
    fn _base_king_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let colo = colour(piece);
        let mut possible_moves: Vec<Move> = Vec::new();
        if row<=6 && colour(self.board[col][row+1]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col,row+1]})}
        if row>=1 && colour(self.board[col][row-1]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col,row-1]})}
        if col<=6 && colour(self.board[col+1][row]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1,row]})}
        if col>=1 && colour(self.board[col-1][row]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1,row]})}
        if row<=6 && col<=6 && colour(self.board[col+1][row+1]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1,row+1]})}
        if row>=1 && col>=1 && colour(self.board[col-1][row-1]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1,row-1]})}
        if row<=6 && col>=1 && colour(self.board[col-1][row+1]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1,row+1]})}
        if row>=1 && col<=6 && colour(self.board[col+1][row-1]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1,row-1]})}
        possible_moves
    }

    #[inline(always)]
    fn _castle_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut possible_moves: Vec<Move> = Vec::new();
        let colo = colour(piece);
        if self.castle[colour_to_index(colo)][0] && row == 4 && self.board[col][1] == EMPTY && self.board[col][2] == EMPTY && self.board[col][3] == EMPTY {
            if !self.check_for_check_static(sq, colo) && !self.check_for_check_static([col, 1], colo) && !self.check_for_check_static([col, 2], colo) && !self.check_for_check_static([col, 3], colo) {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col,2]})
            }
        }
        if self.castle[colour_to_index(colo)][1] && row == 4 && self.board[col][5] == EMPTY && self.board[col][6] == EMPTY {
            if !self.check_for_check_static(sq, colo) && !self.check_for_check_static([col, 5], colo) && !self.check_for_check_static([col, 6], colo) {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col,6]})
            }
        }
        possible_moves
    }

    fn _king_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let mut possible_moves = self._base_king_moves(sq, piece);
        let mut additional_moves = self._castle_moves(sq,piece);
        possible_moves.append(&mut additional_moves);
        possible_moves
    }

    fn _rook_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut possible_moves: Vec<Move> = Vec::new();
        let colo = colour(piece);
        for drow in 1..(row+1) {
            if row<drow { break }
            if colour(self.board[col][row - drow]) != colo {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col,row-drow]});
                if colour(self.board[col][row - drow]) != EMPTY { break }
            }
            else { break }
        }
        for drow in 1..(8-row) {
            if row+drow>=8 { break }
            if colour(self.board[col][row + drow]) != colo {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col,row+drow]});
                if colour(self.board[col][row + drow]) != EMPTY { break }
            }
            else { break }
        }
        for dcol in 1..(col+1) {
            if col<dcol { break }
            if colour(self.board[col-dcol][row]) != colo {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col-dcol,row]});
                if colour(self.board[col-dcol][row]) != EMPTY { break }
            }
            else { break }
        }
        for dcol in 1..(8-col) {
            if col+dcol>=8 { break }
            if colour(self.board[col+dcol][row]) != colo {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col+dcol,row]});
                if colour(self.board[col+dcol][row]) != EMPTY { break }
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
        let colo = colour(piece);
        for change in 1..(smaller+1) {
            if colour(self.board[col-change][row-change]) != colo {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col-change,row-change]});
                if colour(self.board[col-change][row-change]) != EMPTY { break }
            }
            else { break }
        }
        smaller = min(7-col, row);
        for change in 1..(smaller+1) {
            if colour(self.board[col+change][row-change]) != colo {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col+change,row-change]});
                if colour(self.board[col+change][row-change]) != EMPTY { break }
            }
            else { break }
        }
        smaller = min(col, 7-row);
        for change in 1..(smaller+1) {
            if colour(self.board[col-change][row+change]) != colo {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col-change,row+change]});
                if colour(self.board[col-change][row+change]) != EMPTY { break }
            }
            else { break }
        }
        smaller = min(7-col, 7-row);
        for change in 1..(smaller+1) {
            if colour(self.board[col+change][row+change]) != colo {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col+change,row+change]});
                if colour(self.board[col+change][row+change]) != EMPTY { break }
            }
            else { break }
        }
        possible_moves
    }

    fn _knight_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut possible_moves: Vec<Move> = Vec::new();
        let colo = colour(piece);
        if row<=5 && col<=6 && colour(self.board[col+1][row+2]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1,row+2] })}
        if row>=2 && col<=6 && colour(self.board[col+1][row-2]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1,row-2] })}
        if row<=5 && col>=1 && colour(self.board[col-1][row+2]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1,row+2] })}
        if row>=2 && col>=1 && colour(self.board[col-1][row-2]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1,row-2] })}
        if row<=6 && col<=5 && colour(self.board[col+2][row+1]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col+2,row+1] })}
        if row>=1 && col<=5 && colour(self.board[col+2][row-1]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col+2,row-1] })}
        if row<=6 && col>=2 && colour(self.board[col-2][row+1]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col-2,row+1] })}
        if row>=1 && col>=2 && colour(self.board[col-2][row-1]) != colo {possible_moves.push(Move { target: piece, orig: sq, dest: [col-2,row-1] })}
        possible_moves
    }
    #[inline(always)]
    fn _queen_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let mut possible_moves = self._rook_moves(sq, piece);
        let mut additional_moves = self._bishop_moves(sq,piece);
        possible_moves.append(&mut additional_moves);
        possible_moves
    }

    fn unvalidated_moves(&self, sq: Square) -> Vec<Move> {
        let piece = self.board[sq[0]][sq[1]];
        if colour(piece) != self.color {panic!("Not a valid piece")}
        match name(piece) {
            PAWN => self._pawn_moves(sq, piece),
            QUEEN => self._queen_moves(sq, piece),
            ROOK => self._rook_moves(sq, piece),
            KNIGHT => self._knight_moves(sq, piece),
            BISHOP => self._bishop_moves(sq, piece),
            KING => self._king_moves(sq, piece),
            _ => panic!("Not a valid piece!")
        }
    }

    fn _find_all_unvalidated_moves(&self) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
        for column in 0..8 {
            for row in 0..8 {
                if colour(self.board[column][row]) == self.color {
                    let mut current_moves = self.unvalidated_moves([column,row]);
                    possible_moves.append(&mut current_moves);
                }
            }
        }
        possible_moves
    }

    pub fn check_for_check_static(&self, king_square: Square, colour: u8) -> bool {
        let alt_color = other_colour(colour);
        for pos in self._pawn_moves(king_square, colour) {
            if self.board[pos.dest[0]][pos.dest[1]] == PAWN | alt_color {
                return true
            }
        }
        for pos in self._knight_moves(king_square, colour) {
            if self.board[pos.dest[0]][pos.dest[1]] == KNIGHT | alt_color {
                return true
            }
        }
        for pos in self._rook_moves(king_square, colour) {
            if (self.board[pos.dest[0]][pos.dest[1]] == ROOK | alt_color) || (self.board[pos.dest[0]][pos.dest[1]] == QUEEN | alt_color) {
                return true 
            }
        }
        for pos in self._bishop_moves(king_square, colour) {
            if (self.board[pos.dest[0]][pos.dest[1]] == BISHOP | alt_color) || (self.board[pos.dest[0]][pos.dest[1]] == QUEEN | alt_color) {
                return true 
            }
        }
        for pos in self._base_king_moves(king_square,colour) {
            if self.board[pos.dest[0]][pos.dest[1]] == KING | alt_color {
                return true 
            }
        }
        false
    }

    pub fn check_for_check(&self, m: Move, king_square: Square, colour: u8) -> bool {
        let mut temp = self.clone();
        temp.pseudo_move(m);
        temp.color = colour;
        let alt_color = other_colour(temp.color);
        for pos in temp._pawn_moves(king_square, colour) {
            if temp.board[pos.dest[0]][pos.dest[1]] == PAWN | alt_color {
                return true
            }
        }
        for pos in temp._knight_moves(king_square, colour) {
            if temp.board[pos.dest[0]][pos.dest[1]] == KNIGHT | alt_color {
                return true
            }
        }
        for pos in temp._rook_moves(king_square, colour) {
            if (temp.board[pos.dest[0]][pos.dest[1]] == ROOK | alt_color) || (temp.board[pos.dest[0]][pos.dest[1]] == QUEEN | alt_color) {
                return true 
            }
        }
        for pos in temp._bishop_moves(king_square, colour) {
            if (temp.board[pos.dest[0]][pos.dest[1]] == BISHOP | alt_color) || (temp.board[pos.dest[0]][pos.dest[1]] == QUEEN | alt_color) {
                return true 
            }
        }
        for pos in self._base_king_moves(king_square,colour) {
            if self.board[pos.dest[0]][pos.dest[1]] == KING | alt_color {
                return true 
            }
        }
        false
    }

    #[inline(always)]
    fn possible_moves(&self, sq: Square, king_square: Square, colour: u8) -> Vec<Move> {
        let unvalidated = self.unvalidated_moves(sq);
        let mut possible_moves: Vec<Move> = Vec::new();
        if self.board[sq[0]][sq[1]] == KING | colour {
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

    pub fn _selection_possible_moves(&self, sq: Square) -> Vec<Move> {
        let king_square = self.get_king_square(self.color);
        self.possible_moves(sq, king_square, self.color)
    }

    #[inline(always)]
    pub fn find_all_possible_moves(&self) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
        let king_square = self.get_king_square(self.color);
        for column in 0..8 {
            for row in 0..8 {
                if colour(self.board[column][row]) == self.color {
                    let mut current_moves = self.possible_moves([column,row], king_square, self.color);
                    possible_moves.append(&mut current_moves);
                }
            }
        }
        possible_moves
    }

    pub fn _perft(&mut self, depth_left: u8) -> u64 {
        let moves = self.find_all_possible_moves();
        let mut positions: u64 = 0;
        if depth_left == 0 {
            return 1
        }
        if depth_left == 1 {
            for _ in &moves { 
                positions += 1 
            }
            return positions
        }
        for m in moves {
            let pen_castle = self.castle;
            let pen_move = self.last_move;
            let capture = self.board[m.dest[0]][m.dest[1]];
            self.pseudo_move(m);
            positions += self._perft(depth_left-1);
            self.unmake_move(m, pen_move, pen_castle, capture);
        }  
        positions
    }

    pub fn _root_perft(&mut self, depth_left: u8) -> u64 {
        let mut new_move_list: Vec<(Move, u64)> = Vec::new();
        let move_list = self.find_all_possible_moves();
        for mo in move_list {
            let pen_castle = self.castle;
            let pen_move = self.last_move;
            let capture = self.board[mo.dest[0]][mo.dest[1]];
            self.pseudo_move(mo);
            let score = self._perft(depth_left-1);
            new_move_list.push((mo, score));
            println!("{}: {}", mo._to_uci_string(), score);
            self.unmake_move(mo, pen_move, pen_castle, capture);
        }
        let mut positions: u64 = 0;
        for sm in new_move_list {
            positions += sm.1;
        }
        positions
    }
}
