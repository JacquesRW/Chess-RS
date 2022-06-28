//* Support for getting positions from Forsyth-Edwards Notation (FEN) */
//* this is not good code */

use crate::model::defs::*;
use crate::model::pieces::*;
use crate::model::helper::*;

fn piece(ch: char) -> Piece {
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
    match s {
        "w" => WHITE,
        "b" => BLACK,
        _ => panic!("Not valid colour!")
    }
}

fn en_passant_square(s: &str, c: Piece) -> Move {
    if s == "-" { return Move::null() }
    let sq = get_square(&s);
    let x = match c {
        WHITE => -1,
        BLACK => 1,
        _ => panic!("Not valid colour!")
    };
    Move { target: other_colour(c) | PAWN, orig: [(sq[0] as i32 - x) as usize ,sq[1]], dest: [(sq[0] as i32 + x) as usize,sq[1]]}
}

fn can_castle(s: &str) -> [[bool;2];2] {
    if s == "-" { return [[true, true], [true, true]]}
    let mut castle = [[false, false], [false, false]];
    for ch in s.chars() {
        match ch {
            'Q' => castle[0][0] = true,
            'K' => castle[0][1] = true,
            'q' => castle[1][0] = true,
            'k' => castle[1][1] = true,
            _ => panic!("Not good castle info!")
        }
    }
    castle
}

fn convert_row(s: &str) -> [Piece;8] {
    let mut counter = 0;
    let mut row: [Piece;8] = [0;8];
    for ch in s.chars() {
        if !ch.is_numeric() {
            row[counter] = piece(ch);
            counter += 1;
        }
        else {
            let len = (ch as usize) - 48;
            for i in 0..len {
                row[counter + i] = EMPTY;
            }
            counter += len;
        }
    }
    row
}

impl Board {
    pub fn from_fen(s: &str) -> Board {
        let vec: Vec<&str> = s.split_whitespace().collect();
        let rows: Vec<&str> = vec[0].split("/").collect();
        let mut position: Array = [[0;8];8];
        for i in 0..8 {
            position[i] = convert_row(rows[7 - i]);
        }
        let c = find_color(vec[1]);
        let mut board = Board {
            board: position,
            color: c,
            last_move: en_passant_square(vec[3], c), 
            castle: can_castle(vec[2]),
            kings: [[0,0],[0,0]],
        };
        board.kings = [board.get_king_square(WHITE),board.get_king_square(BLACK)];
        board
    }
}
