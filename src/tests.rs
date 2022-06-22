#[cfg(test)]
mod test {
    use crate::model::structs::{Move,Board, Piece};

    #[test]
    fn castling_kingside() {
        println!("Testing kingside castling.");
        let mut game = Board::new();
        game.make_move(Move { target: Piece { piece: 'P', color: 1}, orig: [1,4], dest: [3,4]});
        game.make_move(Move { target: Piece { piece: 'P', color: 2}, orig: [6,4], dest: [4,4]});
        game.make_move(Move { target: Piece { piece: 'B', color: 1}, orig: [0,5], dest: [1,4]});
        game.make_move(Move { target: Piece { piece: 'B', color: 2}, orig: [7,5], dest: [6,4]});
        game.make_move(Move { target: Piece { piece: 'N', color: 1}, orig: [0,6], dest: [2,5]});
        game.make_move(Move { target: Piece { piece: 'N', color: 2}, orig: [7,6], dest: [5,5]});
        game.log();
        game.make_move(Move { target: Piece { piece: 'K', color: 1}, orig: [0,4], dest: [0,6]});
        game.log();
        game.make_move(Move { target: Piece { piece: 'K', color: 2}, orig: [7,4], dest: [7,6]});
        game.log();
    }
    #[test]
    fn castling_queenside() {
        println!("Testing queenside castling.");
        let mut game = Board::new();
        game.make_move(Move { target: Piece { piece: 'P', color: 1}, orig: [1,3], dest: [3,3]});
        game.make_move(Move { target: Piece { piece: 'P', color: 2}, orig: [6,3], dest: [4,3]});
        game.make_move(Move { target: Piece { piece: 'B', color: 1}, orig: [0,2], dest: [2,4]});
        game.make_move(Move { target: Piece { piece: 'B', color: 2}, orig: [7,2], dest: [5,4]});
        game.make_move(Move { target: Piece { piece: 'N', color: 1}, orig: [0,1], dest: [2,2]});
        game.make_move(Move { target: Piece { piece: 'N', color: 2}, orig: [7,1], dest: [5,2]});
        game.make_move(Move { target: Piece { piece: 'Q', color: 1}, orig: [0,3], dest: [1,3]});
        game.make_move(Move { target: Piece { piece: 'Q', color: 2}, orig: [7,3], dest: [6,3]});
        game.log();
        game.make_move(Move { target: Piece { piece: 'K', color: 1}, orig: [0,4], dest: [0,2]});
        game.log();
        game.make_move(Move { target: Piece { piece: 'K', color: 2}, orig: [7,4], dest: [7,2]});
        game.log();
    }
    #[test]
    fn white_en_passant() {
        println!("Testing en passant.");
        let mut game = Board::new();
        game.make_move(Move { target: Piece { piece: 'P', color: 1}, orig: [1,4], dest: [3,4]});
        game.make_move(Move { target: Piece { piece: 'P', color: 2}, orig: [6,4], dest: [5,4]});
        game.make_move(Move { target: Piece { piece: 'P', color: 1}, orig: [3,4], dest: [4,4]});
        game.make_move(Move { target: Piece { piece: 'P', color: 2}, orig: [6,3], dest: [4,3]});
        game.log();
        game.make_move(Move { target: Piece { piece: 'P', color: 1}, orig: [4,4], dest: [5,3]});
        game.log();
    }
    #[test]
    fn black_en_passant() {
        println!("Testing en passant.");
        let mut game = Board::new();
        game.make_move(Move { target: Piece { piece: 'P', color: 1}, orig: [1,4], dest: [2,4]});
        game.make_move(Move { target: Piece { piece: 'P', color: 2}, orig: [6,4], dest: [4,4]});
        game.make_move(Move { target: Piece { piece: 'P', color: 1}, orig: [1,2], dest: [2,2]});
        game.make_move(Move { target: Piece { piece: 'P', color: 2}, orig: [4,4], dest: [3,4]});
        game.make_move(Move { target: Piece { piece: 'P', color: 1}, orig: [1,3], dest: [3,3]});
        game.log();
        game.make_move(Move { target: Piece { piece: 'P', color: 2}, orig: [3,4], dest: [2,3]});
        game.log();
    }
}