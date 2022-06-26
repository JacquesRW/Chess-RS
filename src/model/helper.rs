//* Useful misc public functions. */
use crate::model::defs::Square;

pub fn get_coords(square: &Square) -> String{
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

pub fn get_square(coord: &str) -> Square {
    let x = coord.as_bytes()[0] as char;
    let y = coord.as_bytes()[1] as char;
    let row = match x {
        'A' | 'a' => 0,
        'B' | 'b' => 1,
        'C' | 'c' => 2,
        'D' | 'd' => 3,
        'E' | 'e' => 4,
        'F' | 'f' => 5,
        'G' | 'g' => 6,
        'H' | 'h' => 7,
        _ => panic!("Invalid square.")
    };
    let col = match y {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => panic!("Invalid square.")
    };
    return [col, row]
}
