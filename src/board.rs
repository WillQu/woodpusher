use regex::Regex;
use im::hashmap::HashMap;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Piece {
    piece_type: PieceType,
    player: Player,
}

impl Piece {
    pub fn new(piece_type: PieceType, player: Player) -> Piece {
        Piece {piece_type: piece_type, player: player}
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            White => Black,
            Black => White,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Board {
    squares: HashMap<Position, Piece>,
}

use self::PieceType::*;
use self::Player::*;
impl Board {

    pub fn starting_position() -> Board {
        let mut board_map = HashMap::new();
        board_map.insert(Position::from("e1").unwrap(), Piece {
            piece_type: King,
            player: White,
        });
        board_map.insert(Position::from("e8").unwrap(), Piece {
            piece_type: King,
            player: Black,
        });
        board_map.insert(Position::from("d1").unwrap(), Piece {
            piece_type: Queen,
            player: White,
        });
        board_map.insert(Position::from("d8").unwrap(), Piece {
            piece_type: Queen,
            player: Black,
        });
        board_map.insert(Position::from("c1").unwrap(), Piece {
            piece_type: Bishop,
            player: White,
        });
        board_map.insert(Position::from("c8").unwrap(), Piece {
            piece_type: Bishop,
            player: Black,
        });
        board_map.insert(Position::from("f1").unwrap(), Piece {
            piece_type: Bishop,
            player: White,
        });
        board_map.insert(Position::from("f8").unwrap(), Piece {
            piece_type: Bishop,
            player: Black,
        });
        board_map.insert(Position::from("b1").unwrap(), Piece {
            piece_type: Knight,
            player: White,
        });
        board_map.insert(Position::from("b8").unwrap(), Piece {
            piece_type: Knight,
            player: Black,
        });
        board_map.insert(Position::from("g1").unwrap(), Piece {
            piece_type: Knight,
            player: White,
        });
        board_map.insert(Position::from("g8").unwrap(), Piece {
            piece_type: Knight,
            player: Black,
        });
        board_map.insert(Position::from("a1").unwrap(), Piece {
            piece_type: Rook,
            player: White,
        });
        board_map.insert(Position::from("a8").unwrap(), Piece {
            piece_type: Rook,
            player: Black,
        });
        board_map.insert(Position::from("h1").unwrap(), Piece {
            piece_type: Rook,
            player: White,
        });
        board_map.insert(Position::from("h8").unwrap(), Piece {
            piece_type: Rook,
            player: Black,
        });
        for c in "abcdefgh".chars() {
            board_map.insert(Position::from(&format!("{}2", c)).unwrap(), Piece {
                piece_type: Pawn,
                player: White,
            });
            board_map.insert(Position::from(&format!("{}7", c)).unwrap(), Piece {
                piece_type: Pawn,
                player: Black,
            });
        }
        Board {
            squares: board_map
        }
    }

    pub fn get(&self, coordinates: &str) -> Option<&Piece> {
        Position::from(coordinates).and_then(|pos| self.squares.get(&pos))
    }

    pub fn put(&self, coordinates: &str, piece: Piece) -> Board {
        Position::from(coordinates)
            .map_or_else(|| self.clone(), |pos| Board{squares: self.squares.update(pos, piece)})
        
    }

    pub fn remove(&self, coordinates: &str) -> Board {
        Position::from(coordinates)
            .map_or_else(|| self.clone(), |pos| Board {squares: self.squares.without(&pos)})
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position {
    pos: String,
}


impl Position {

    fn from(pos: &str) -> Option<Position> {
        lazy_static! {
            static ref POS_REGEX: Regex = Regex::new("^[a-h][1-8]$").unwrap();
        }
        if POS_REGEX.is_match(pos) {
            Some(Position{pos: String::from(pos)})
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use board::Board;
    use board::Piece;
    use board::PieceType::*;
    use board::Player::*;
    use board::Position;

    #[test]
    fn new_board() {
        // When
        let result = Board::starting_position();

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

    #[test]
    fn empty_pos() {
        // Given
        let pos = "";

        // When
        let result = Position::from(pos);

        // Then
        assert_eq!(result, None);
    }

    #[test]
    fn ok_pos() {
        // Given
        let pos = "a1";

        // When
        let result = Position::from(pos);

        // Then
        assert_eq!(result, Some(Position{pos: String::from("a1")}));
    }

    #[test]
    fn invalid_pos() {
        // Given
        let pos = "~|";

        // When
        let result = Position::from(pos);

        // Then
        assert_eq!(result, None);
    }
}
