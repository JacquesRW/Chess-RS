use crate::model::defs::*;
use crate::icons::*;
use crate::model::pieces::*;
use crate::WIDTH;
use std::cmp::{max,min};

fn pixel_art(pc: Piece) -> [[u8;8];8] {
     match name(pc) {
        PAWN => PAWN_ICON,
        ROOK => ROOK_ICON,
        QUEEN => QUEEN_ICON,
        KING => KING_ICON,
        KNIGHT => KNIGHT_ICON,
        BISHOP => BISHOP_ICON,
        EMPTY => EMPTY_ICON,
        _ => panic!("Invalid piece!")
    }
}

pub fn draw_square(game: &Board, x: usize, y: usize, screen: &mut [u8]) {
    let inx0 = (7 - x) * WIDTH as usize * 32 + y * 32; 
    let pc = game.board[x][y];
    let fill = match colour(pc) { WHITE => [0xff, 0xff, 0xff, 0xff], BLACK => [0, 0, 0, 0], EMPTY => [0, 0, 0, 0],_ => panic!("Invalid fill match.")};
    let b = ( max(x,y) - min(x,y) ) % 2;
    let background = match  b {1 => [194, 178, 128, 0xff], 0 => [90, 51, 0, 255], _ => panic!("Invalid background colour match with {}.", b)};
    let data = pixel_art(pc);
    for i in 0..8 {
        for j in 0..8 {
            let inx1 = inx0 + j*4 + i * WIDTH as usize * 4;
            if data[i][j] == 1 {
                for k in 0..4 {
                    screen[inx1+k] = fill[k];
                }
            }
            if data[i][j] == 0 {
                for k in 0..4 {
                    screen[inx1+k] = background[k];
                }
            }
        }
    }
}