//* Support for getting positions from Forsyth-Edwards Notation (FEN) */

use crate::model::defs::*;
use crate::model::engine::eval::phase_value;
use crate::model::pieces::*;
use crate::model::helper::*;

fn piece(ch: char) -> Piece {
    // matches character to the corresponding piece
    match ch {
    'p' => BLACK | PAWN,
    'P' => WHITE | PAWN,
    'n' => BLACK | KNIGHT,
    'N' => WHITE | KNIGHT,
    'b' => BLACK | BISHOP,
    'B' => WHITE | BISHOP,
    'r' => BLACK | ROOK,
    'R' => WHITE | ROOK,
    'q' => BLACK | QUEEN,
    'Q' => WHITE | QUEEN,
    'k' => BLACK | KING,
    'K' => WHITE | KING,
    _ => panic!("Not valid!")
    }
}

fn find_color(s: &str) -> Piece {
    // finds the side to move
    match s {
        "w" => WHITE,
        "b" => BLACK,
        _ => panic!("Not valid colour!")
    }
}

fn en_passant_square(s: &str, c: Piece) -> Move {
    // generates last move if relevant en passant square, otherwise null
    if s == "-" { return Move::null() }
    let sq = get_square(&s);
    let x = match c {
        WHITE => -1,
        BLACK => 1,
        _ => panic!("Not valid colour!")
    };
    Move { target: other_colour(c) | PAWN, orig: [(sq[0] as i32 - x) as usize ,sq[1]], dest: [(sq[0] as i32 + x) as usize,sq[1]]}
}

fn can_castle(s: &str) -> Castle {
    // finds castling rights
    if s == "-" { return NO_RIGHTS}
    let mut castle = NO_RIGHTS;
    for ch in s.chars() {
        match ch {
            'Q' => castle |= WHITE_QS,
            'K' => castle |= WHITE_KS,
            'q' => castle |= BLACK_QS,
            'k' => castle |= BLACK_KS,
            _ => panic!("Not good castle info!")
        }
    }
    castle
}

fn convert_row(s: &str) -> [Piece;8] {
    // converts a row string to its Board representation
    let mut counter = 0;
    let mut row: [Piece;8] = [0;8];
    for ch in s.chars() {
        // if piece, insert it
        if !ch.is_numeric() {
            row[counter] = piece(ch);
            counter += 1;
        }
        // skip empty squares
        else {
            let len = (ch as usize) - 48;
            for i in 0..len {
                row[counter + i] = EMPTY;
            }
            counter += len;
        }
    }
    // returns row
    row
}

fn get_phase(board: Array) -> i64 {
    let mut eval: i64 = 0;
    for i in 0..8 {
        for j in 0..8 {
            eval += phase_value(board[i][j]);               
        }
    }
    eval as i64 
}

impl Board {
    pub fn from_fen(s: &str) -> Board {
        // splits fen string by whitespace
        let vec: Vec<&str> = s.split_whitespace().collect();
        // gets rows of board
        let rows: Vec<&str> = vec[0].split("/").collect();
        // initialise board
        let mut position: Array = [[0;8];8];
        // filling in board
        for i in 0..8 {
            position[i] = convert_row(rows[7 - i]);
        }
        // getting side to move
        let c = find_color(vec[1]);
        // making Board instance
        let mut board = Board {
            board: position,
            color: c,
            last_move: en_passant_square(vec[3], c), 
            castle: can_castle(vec[2]),
            kings: [[0,0],[0,0]],
            phase: get_phase(position)
        };
        // get king positions
        board.kings = [board.get_king_square(WHITE),board.get_king_square(BLACK)];
        // returns board
        board
    }
}
