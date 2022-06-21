#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    // piece = 'e','P','K','Q','R','N' or 'B'
    pub piece: char,
    // color = 0,1,2
    pub color: usize
}

pub type Board = [[Piece;8];8];
pub type Square = [usize;2];

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub target: Piece,
    pub orig: Square,
    pub dest: Square 
}

pub fn get_coords(square: Square) -> String{
    let x = match square[1] {
        0 => String::from("A"),
        1 => String::from("B"),
        2 => String::from("C"),
        3 => String::from("D"),
        4 => String::from("E"),
        5 => String::from("F"),
        6 => String::from("G"),
        7 => String::from("H"),
        _ => panic!("Square out of board.")
    };
    format!("{}{}", x, (square[0] + 1).to_string())
}

pub fn other_colour(color: usize) -> usize{
    match color {
        1 => 2,
        2 => 1,
        _ => panic!("Not valid colour.")
    }
}

impl Piece {
    pub fn repr(&self) -> String {
        format!("{}{}", self.piece.to_string(),self.color.to_string())
    }
    pub fn to_string(&self) -> String {
        let x = match &self.piece {
            'P' => "pawn",
            'K' => "king",
            'Q' => "queen",
            'R' => "rook",
            'B' => "bishop",
            'N' => "kinght",
            'e' => "Empty",
            _ => panic!("Not a valid piece")
        };
        let y = match &self.color {
            0 => "",
            1 => "White",
            2 => "Black",
            _ => panic!("Not a valid colour.")
        };
        format!("{y} {x}")
    }
    pub fn empty() -> Piece {
        Piece {piece: 'e', color: 0}
    }
    pub fn king(colour: usize) -> Piece {
        Piece {piece: 'K', color: colour}
    }
    pub fn pawn(colour: usize) -> Piece {
        Piece {piece: 'P', color: colour}
    }
    pub fn knight(colour: usize) -> Piece {
        Piece {piece: 'N', color: colour}
    }
    pub fn bishop(colour: usize) -> Piece {
        Piece {piece: 'B', color: colour}
    }
    pub fn rook(colour: usize) -> Piece {
        Piece {piece: 'R', color: colour}
    }
    pub fn queen(colour: usize) -> Piece {
        Piece {piece: 'Q', color: colour}
    }
}

impl Move {

    pub fn null() -> Move {
        Move {target: Piece::empty(), orig: [0,0], dest: [0,0]}
    }
    pub fn new(piece: Piece, origin: Square, destination: Square) -> Move {
        Move {target: piece, orig: origin, dest: destination}
    }

    pub fn log(&self) {
        println!("{}, {} to {}", self.target.to_string(), get_coords(self.orig), get_coords(self.dest))
    }

}
