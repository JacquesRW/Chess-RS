pub mod types;
use types::*;

pub struct Game {
    pub board: Board,
    last_move: Move,
    castle: [[bool;2];3],
    color: usize
}

impl Game {
    pub fn new() -> Game {
        Game {
        board: [[Piece::rook(1),Piece::knight(1),Piece::bishop(1),Piece::queen(1),Piece::king(1),Piece::bishop(1),Piece::knight(1),Piece::rook(1)],
        [Piece::pawn(1),Piece::pawn(1),Piece::pawn(1),Piece::pawn(1),Piece::pawn(1),Piece::pawn(1),Piece::pawn(1),Piece::pawn(1)],
        [Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty()],
        [Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty()],
        [Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty()],
        [Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty(),Piece::empty()],
        [Piece::pawn(2),Piece::pawn(2),Piece::pawn(2),Piece::pawn(2),Piece::pawn(2),Piece::pawn(2),Piece::pawn(2),Piece::pawn(2)],
        [Piece::rook(2),Piece::knight(2),Piece::bishop(2),Piece::queen(2),Piece::king(2),Piece::bishop(2),Piece::knight(2),Piece::rook(2)]], 
        
        last_move: Move::null(), 
        //  castle = [
        //  [has p1 castled, has p2 castled],
        //  [can p1 castle left, can p1 castle right],
        //  [can p2 castle left, can p2 castle right]]
        castle: [[false,false], [true, true], [true, true]], 
        color: 1
        }
    }

    pub fn switch_color(&mut self) {
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
        m.log();
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

    pub fn log(&self) {
        println!("-------------------------");
        let board_ref: &[[Piece;8];8] = &self.board;
        for i in 0..8 {
            let mut line: String = String::from(" ");
            for j in 0..8 {
                line.push_str(&(board_ref[7-i][j].repr()));
                line.push_str(" ");
            }
            println!("{}", line)
        }
        println!("-------------------------");
    }

    fn _pawn_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
        let col = sq[0];
        let row = sq[1];
        if piece.color == 1 {
            if col == 4 {
                if self.last_move == (Move { target: Piece {piece: 'P', color: 2}, orig: [row+1, 6], dest: [row+1, 4] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [5, row+1] });}
                if self.last_move == (Move { target: Piece {piece: 'P', color: 2}, orig: [row-1,6], dest: [row-1, 4] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [5, row-1] });}}
            if self.board[col+1][row] == (Piece {piece: 'e', color: 0}) {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col+1, row] });
                if col == 1 {if self.board[col+2][row] == (Piece {piece: 'e', color: 0}) {possible_moves.push(Move { target: piece, orig: sq, dest: [col+2, row] })}}}
            if row >= 1 {if self.board[col+1][row-1].color == 2 {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1, row-1] })}}
            if row+1 <= 7 {if self.board[col+1][row+1].color == 2 {possible_moves.push(Move { target: piece, orig: sq, dest: [col+1, row+1] })}}
        }
        if piece.color == 2 {
            if col == 3 {
                if self.last_move == (Move { target: Piece {piece: 'P', color: 1}, orig: [row+1, 1], dest: [row+1, 3] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [2, row+1] });}
                if self.last_move == (Move { target: Piece {piece: 'P', color: 1}, orig: [row-1,1], dest: [row-1, 3] }) {
                    possible_moves.push(Move { target: piece, orig: sq, dest: [2, row-1] });}}
            if self.board[col-1][row] == (Piece {piece: 'e', color: 0}) {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col-1, row] });
                if col == 6 {if self.board[col-2][row] == (Piece {piece: 'e', color: 0}) {possible_moves.push(Move { target: piece, orig: sq, dest: [col-2, row] })}}}
            if row >= 1 {if self.board[col-1][row-1].color == 1 {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1, row-1] })}}
            if row+1 <= 7 {if self.board[col-1][row+1].color == 1 {possible_moves.push(Move { target: piece, orig: sq, dest: [col-1, row+1] })}}
        }
        possible_moves
    }

    fn _king_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
        possible_moves
     }
    fn _rook_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let col = sq[0];
        let row = sq[1];
        let mut possible_moves: Vec<Move> = Vec::new();
        for drow in 1..(row+1) {
            if self.board[col][row - drow].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col,row-drow]});
                if self.board[col][row - drow].color != 0 { break }
            }
        }
        for drow in 1..(8-row) {
            if self.board[col][row + drow].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col,row+drow]});
                if self.board[col][row + drow].color != 0 { break }
            }
        }
        for dcol in 1..(col+1) {
            if self.board[col-dcol][row].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col-dcol,row]});
                if self.board[col-dcol][row].color != 0 { break }
            }
        }
        for dcol in 1..(8-row) {
            if self.board[col+dcol][row].color != piece.color {
                possible_moves.push(Move { target: piece, orig: sq, dest: [col+dcol,row]});
                if self.board[col+dcol][row].color != 0 { break }
            }
        }
        possible_moves
    }
    fn _bishop_moves(&self, sq: Square, piece: Piece) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
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

    fn _unvalidated_move(&self, sq: Square) -> Vec<Move> {
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
}
