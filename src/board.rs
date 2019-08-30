use std::fmt;
use std::fmt::Display;

use im::hashmap::HashMap;
use regex::Regex;

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
    pub fn new(piece_type: PieceType, player: Player) -> Self {
        Self { piece_type, player }
    }

    pub fn piece_type(self) -> PieceType {
        self.piece_type
    }

    pub fn player(self) -> Player {
        self.player
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn opponent(self) -> Self {
        match self {
            White => Black,
            Black => White,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Position {
    column: u8,
    row: u8,
}

impl Position {
    pub fn from(pos: &str) -> Option<Self> {
        lazy_static! {
            static ref POS_REGEX: Regex = Regex::new("^[a-h][1-8]$").unwrap();
        }
        if POS_REGEX.is_match(pos) {
            let bytes = pos.as_bytes();
            Some(Self {
                column: bytes[0],
                row: bytes[1],
            })
        } else {
            None
        }
    }

    pub fn from_chars(column: char, row: char) -> Option<Self> {
        if column >= 'a' && column <= 'h' && row >= '1' && row <= '8' {
            Some(Self {
                row: row as u8,
                column: column as u8,
            })
        } else {
            None
        }
    }

    pub fn from_u8(column: u8, row: u8) -> Option<Self> {
        if column >= b'a' && column <= b'h' && row >= b'1' && row <= b'8' {
            Some(Self { row, column })
        } else {
            None
        }
    }

    pub fn column(self) -> u8 {
        self.column
    }

    pub fn row(self) -> u8 {
        self.row
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.column as char, self.row as char)
    }
}

use self::PieceType::*;
use self::Player::*;
#[derive(PartialEq, Debug, Clone)]
pub struct Board {
    squares: HashMap<Position, Piece>,
}

impl Board {
    pub fn starting_position() -> Self {
        lazy_static! {
            static ref STARTING_POS: Board = {
                let mut board_map = HashMap::new();
                board_map.insert(
                    Position::from("e1").unwrap(),
                    Piece {
                        piece_type: King,
                        player: White,
                    },
                );
                board_map.insert(
                    Position::from("e8").unwrap(),
                    Piece {
                        piece_type: King,
                        player: Black,
                    },
                );
                board_map.insert(
                    Position::from("d1").unwrap(),
                    Piece {
                        piece_type: Queen,
                        player: White,
                    },
                );
                board_map.insert(
                    Position::from("d8").unwrap(),
                    Piece {
                        piece_type: Queen,
                        player: Black,
                    },
                );
                board_map.insert(
                    Position::from("c1").unwrap(),
                    Piece {
                        piece_type: Bishop,
                        player: White,
                    },
                );
                board_map.insert(
                    Position::from("c8").unwrap(),
                    Piece {
                        piece_type: Bishop,
                        player: Black,
                    },
                );
                board_map.insert(
                    Position::from("f1").unwrap(),
                    Piece {
                        piece_type: Bishop,
                        player: White,
                    },
                );
                board_map.insert(
                    Position::from("f8").unwrap(),
                    Piece {
                        piece_type: Bishop,
                        player: Black,
                    },
                );
                board_map.insert(
                    Position::from("b1").unwrap(),
                    Piece {
                        piece_type: Knight,
                        player: White,
                    },
                );
                board_map.insert(
                    Position::from("b8").unwrap(),
                    Piece {
                        piece_type: Knight,
                        player: Black,
                    },
                );
                board_map.insert(
                    Position::from("g1").unwrap(),
                    Piece {
                        piece_type: Knight,
                        player: White,
                    },
                );
                board_map.insert(
                    Position::from("g8").unwrap(),
                    Piece {
                        piece_type: Knight,
                        player: Black,
                    },
                );
                board_map.insert(
                    Position::from("a1").unwrap(),
                    Piece {
                        piece_type: Rook,
                        player: White,
                    },
                );
                board_map.insert(
                    Position::from("a8").unwrap(),
                    Piece {
                        piece_type: Rook,
                        player: Black,
                    },
                );
                board_map.insert(
                    Position::from("h1").unwrap(),
                    Piece {
                        piece_type: Rook,
                        player: White,
                    },
                );
                board_map.insert(
                    Position::from("h8").unwrap(),
                    Piece {
                        piece_type: Rook,
                        player: Black,
                    },
                );
                for c in "abcdefgh".chars() {
                    board_map.insert(
                        Position::from(&format!("{}2", c)).unwrap(),
                        Piece {
                            piece_type: Pawn,
                            player: White,
                        },
                    );
                    board_map.insert(
                        Position::from(&format!("{}7", c)).unwrap(),
                        Piece {
                            piece_type: Pawn,
                            player: Black,
                        },
                    );
                }
                Board { squares: board_map }
            };
        }
        STARTING_POS.clone()
    }

    pub fn empty() -> Self {
        Self {
            squares: HashMap::new(),
        }
    }

    pub fn get(&self, position: Position) -> Option<&Piece> {
        self.squares.get(&position)
    }

    pub fn put(&self, position: Position, piece: Piece) -> Self {
        Self {
            squares: self.squares.update(position, piece),
        }
    }

    pub fn remove(&self, position: Position) -> Self {
        Self {
            squares: self.squares.without(&position),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Position, Piece)> {
        self.squares.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Board;
    use super::Piece;
    use super::PieceType::*;
    use super::Player::*;
    use super::Position;

    #[test]
    fn new_board() {
        // When
        let result = Board::starting_position();

        // Then
        assert_eq!(
            result.get(Position::from("e1").unwrap()),
            Some(&Piece {
                piece_type: King,
                player: White
            })
        );
        assert_eq!(
            result.get(Position::from("e8").unwrap()),
            Some(&Piece {
                piece_type: King,
                player: Black
            })
        );
        assert_eq!(
            result.get(Position::from("d1").unwrap()),
            Some(&Piece {
                piece_type: Queen,
                player: White
            })
        );
        assert_eq!(
            result.get(Position::from("d8").unwrap()),
            Some(&Piece {
                piece_type: Queen,
                player: Black
            })
        );
        assert_eq!(
            result.get(Position::from("c1").unwrap()),
            Some(&Piece {
                piece_type: Bishop,
                player: White
            })
        );
        assert_eq!(
            result.get(Position::from("c8").unwrap()),
            Some(&Piece {
                piece_type: Bishop,
                player: Black
            })
        );
        assert_eq!(
            result.get(Position::from("f1").unwrap()),
            Some(&Piece {
                piece_type: Bishop,
                player: White
            })
        );
        assert_eq!(
            result.get(Position::from("f8").unwrap()),
            Some(&Piece {
                piece_type: Bishop,
                player: Black
            })
        );
        assert_eq!(
            result.get(Position::from("b1").unwrap()),
            Some(&Piece {
                piece_type: Knight,
                player: White
            })
        );
        assert_eq!(
            result.get(Position::from("b8").unwrap()),
            Some(&Piece {
                piece_type: Knight,
                player: Black
            })
        );
        assert_eq!(
            result.get(Position::from("g1").unwrap()),
            Some(&Piece {
                piece_type: Knight,
                player: White
            })
        );
        assert_eq!(
            result.get(Position::from("g8").unwrap()),
            Some(&Piece {
                piece_type: Knight,
                player: Black
            })
        );
        assert_eq!(
            result.get(Position::from("a1").unwrap()),
            Some(&Piece {
                piece_type: Rook,
                player: White
            })
        );
        assert_eq!(
            result.get(Position::from("a8").unwrap()),
            Some(&Piece {
                piece_type: Rook,
                player: Black
            })
        );
        assert_eq!(
            result.get(Position::from("h1").unwrap()),
            Some(&Piece {
                piece_type: Rook,
                player: White
            })
        );
        assert_eq!(
            result.get(Position::from("h8").unwrap()),
            Some(&Piece {
                piece_type: Rook,
                player: Black
            })
        );
        for c in "abcdefgh".chars() {
            assert_eq!(
                result.get(Position::from(&format!("{}2", c)).unwrap()),
                Some(&Piece {
                    piece_type: Pawn,
                    player: White
                })
            );
            assert_eq!(
                result.get(Position::from(&format!("{}7", c)).unwrap()),
                Some(&Piece {
                    piece_type: Pawn,
                    player: Black
                })
            );
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
        assert_eq!(result.unwrap().to_string(), "a1");
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
