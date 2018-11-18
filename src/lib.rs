#[derive(PartialEq, Debug)]
enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(PartialEq, Debug)]
struct Piece {
    piece_type: PieceType,
    player: Player,
}

#[derive(PartialEq, Debug)]
enum Player {
    White,
    Black,
}

struct Board {
    squares: HashMap<String, Piece>,
}

use PieceType::*;
use Player::*;
use std::collections::HashMap;
impl Board {

    fn new_board() -> Board {
        let mut board_map = HashMap::new();
        board_map.insert(String::from("e1"), Piece {
            piece_type: King,
            player: White,
        });
        board_map.insert(String::from("e8"), Piece {
            piece_type: King,
            player: Black,
        });
        board_map.insert(String::from("d1"), Piece {
            piece_type: Queen,
            player: White,
        });
        board_map.insert(String::from("d8"), Piece {
            piece_type: Queen,
            player: Black,
        });
        board_map.insert(String::from("c1"), Piece {
            piece_type: Bishop,
            player: White,
        });
        board_map.insert(String::from("c8"), Piece {
            piece_type: Bishop,
            player: Black,
        });
        board_map.insert(String::from("f1"), Piece {
            piece_type: Bishop,
            player: White,
        });
        board_map.insert(String::from("f8"), Piece {
            piece_type: Bishop,
            player: Black,
        });
        board_map.insert(String::from("b1"), Piece {
            piece_type: Knight,
            player: White,
        });
        board_map.insert(String::from("b8"), Piece {
            piece_type: Knight,
            player: Black,
        });
        board_map.insert(String::from("g1"), Piece {
            piece_type: Knight,
            player: White,
        });
        board_map.insert(String::from("g8"), Piece {
            piece_type: Knight,
            player: Black,
        });
        board_map.insert(String::from("a1"), Piece {
            piece_type: Rook,
            player: White,
        });
        board_map.insert(String::from("a8"), Piece {
            piece_type: Rook,
            player: Black,
        });
        board_map.insert(String::from("h1"), Piece {
            piece_type: Rook,
            player: White,
        });
        board_map.insert(String::from("h8"), Piece {
            piece_type: Rook,
            player: Black,
        });
        for c in "abcdefgh".chars() {
            board_map.insert(format!("{}2", c), Piece {
                piece_type: Pawn,
                player: White,
            });
            board_map.insert(format!("{}7", c), Piece {
                piece_type: Pawn,
                player: Black,
            });
        }
        Board {
            squares: board_map
        }
    }

    fn get(&self, coordinates: &str) -> Option<&Piece> {
        self.squares.get(coordinates)
    }
}

#[cfg(test)]
mod tests {
    use Board;
    use Piece;
    use PieceType::*;
    use Player::*;

    #[test]
    fn new_board() {
        // When
        let result = Board::new_board();

        // Then
        assert_eq!(result.get("e1"), Some(& Piece { piece_type: King, player: White}));
        assert_eq!(result.get("e8"), Some(& Piece { piece_type: King, player: Black}));
        assert_eq!(result.get("d1"), Some(& Piece { piece_type: Queen, player: White}));
        assert_eq!(result.get("d8"), Some(& Piece { piece_type: Queen, player: Black}));
        assert_eq!(result.get("c1"), Some(& Piece { piece_type: Bishop, player: White}));
        assert_eq!(result.get("c8"), Some(& Piece { piece_type: Bishop, player: Black}));
        assert_eq!(result.get("f1"), Some(& Piece { piece_type: Bishop, player: White}));
        assert_eq!(result.get("f8"), Some(& Piece { piece_type: Bishop, player: Black}));
        assert_eq!(result.get("b1"), Some(& Piece { piece_type: Knight, player: White}));
        assert_eq!(result.get("b8"), Some(& Piece { piece_type: Knight, player: Black}));
        assert_eq!(result.get("g1"), Some(& Piece { piece_type: Knight, player: White}));
        assert_eq!(result.get("g8"), Some(& Piece { piece_type: Knight, player: Black}));
        assert_eq!(result.get("a1"), Some(& Piece { piece_type: Rook, player: White}));
        assert_eq!(result.get("a8"), Some(& Piece { piece_type: Rook, player: Black}));
        assert_eq!(result.get("h1"), Some(& Piece { piece_type: Rook, player: White}));
        assert_eq!(result.get("h8"), Some(& Piece { piece_type: Rook, player: Black}));
        for c in "abcdefgh".chars() {
            assert_eq!(result.get(&format!("{}2", c)), Some(& Piece { piece_type: Pawn, player: White}));
            assert_eq!(result.get(&format!("{}7", c)), Some(& Piece { piece_type: Pawn, player: Black}));
        }
    }
}