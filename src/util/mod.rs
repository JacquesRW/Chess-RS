pub mod piece;
pub mod moves;
use moves::Move;
use piece::Piece;

pub struct Board {
    pub board: [[Piece;8];8],
    last_move: Move,
    castle: [[bool;2];3],
    // player: bool not yet implemented
}

impl Board {
    pub fn new() -> Board {
        Board {
        board: [
            [Piece::rook(1),Piece::knight(1),Piece::bishop(1),Piece::queen(1),Piece::king(1),Piece::bishop(1),Piece::knight(1),Piece::rook(1)],
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
        // player: true
        }
    }

    fn try_en_passant(&mut self, &m: &Move) {
        if m.target == (Piece {piece: 'P', color: 1}) {
            if (m.dest[0] == m.orig[0] + 1) && (m.dest[1] == m.orig[1] - 1) {
                self.board[m.orig[0]][m.dest[1]] = Piece::empty()
            }
        }
    }

    fn try_castle(&mut self, &m: &Move) { 
        if m == (Move { target: Piece{piece: 'K', color: 1} , orig: [0,4], dest: [0,2] }) {
            println!("White castles Queenside.");
            self.board[0][3] = Piece::rook(1);
            self.board[0][0] = Piece::empty();
        }
        if m == (Move { target: Piece{piece: 'K', color: 1} , orig: [0,4], dest: [0,6] }) {
            println!("White castles Queenside.");
            self.board[0][5] = Piece::rook(1);
            self.board[0][7] = Piece::empty();
        }
        if m == (Move { target: Piece{piece: 'K', color: 2} , orig: [7,4], dest: [7,2] }) {
            println!("White castles Queenside.");
            self.board[7][3] = Piece::rook(2);
            self.board[7][0] = Piece::empty();
        }
        if m == (Move { target: Piece{piece: 'K', color: 2} , orig: [7,4], dest: [7,6] }) {
            println!("White castles Queenside.");
            self.board[7][5] = Piece::rook(2);
            self.board[7][7] = Piece::empty();
        }
    }

    fn update_castle(&mut self, m: &Move) {
        if m.target.piece == 'K' {
            self.castle[m.target.color] = [false, false]
        }
        if m.target == Piece::rook(1) {
            if m.orig == [0,0] {
                self.castle[m.target.color][0] = false
            }
            if m.orig == [0,7] {
                self.castle[m.target.color][1] = false
            }
        }
        if m.target == Piece::rook(2) {
            if m.orig == [7,0] {
                self.castle[m.target.color][0] = false
            }
            if m.orig == [7,7] {
                self.castle[m.target.color][1] = false
            }
        }
    }

    pub fn make_move(&mut self, m: Move) {
        m.log();
        if m.target != self.board[m.orig[0]][m.orig[1]] {
            panic!("Cannot make this move!")
        }
        self.board[m.dest[0]][m.dest[1]] = m.target;
        self.board[m.orig[0]][m.orig[1]] = Piece::empty();
        self.try_en_passant(&m);
        self.try_castle(&m);
        self.update_castle(&m);
        self.last_move = m;
    }

    pub fn log(&self) {
        println!("-------------------------");
        let board_ref = &self.board;
        for i in 0..8 {
            let mut line = String::from(" ");
            for j in 0..8 {
                line.push_str(&(board_ref[7-i][j].string_form()));
                line.push_str(" ");
            }
            println!("{}", line)
        }
        println!("-------------------------");
    }
}
