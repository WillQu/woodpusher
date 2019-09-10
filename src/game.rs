use im::Vector;

use board::Board;
use board::Piece;
use board::PieceType;
use board::Player;
use board::Position;

mod bishop;
mod king;
mod knight;
mod move_list;
mod pawn;
mod queen;
mod rook;

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    board: Board,
    player_turn: Player,
    en_passant: Option<Position>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::starting_position(),
            player_turn: Player::White,
            en_passant: Option::None,
        }
    }

    pub fn from_board(board: Board, player: Player) -> Self {
        Self {
            board,
            player_turn: player,
            en_passant: Option::None,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    fn turn(&self) -> Player {
        self.player_turn
    }

    pub fn execute_move(&self, from: Position, to: Position) -> Result<Self, String> {
        self.list_moves()
            .iter()
            .find(|mv| mv.from == from && mv.to == to)
            .map(Move::new_game)
            .ok_or_else(|| "Illegal move".to_string())
    }

    fn apply_move_with_en_passant(
        &self,
        from: Position,
        to: Position,
        en_passant: Option<Position>,
    ) -> Result<Self, String> {
        self.get_piece_at(from).map_or_else(
            || Err(format!("No piece at {}", from)),
            |piece| {
                self.apply_move_to_piece(from, to, *piece)
                    .map(|game| Self { en_passant, ..game })
            },
        )
    }

    fn apply_move_to_piece(
        &self,
        from: Position,
        to: Position,
        piece: Piece,
    ) -> Result<Self, String> {
        if piece.player() == self.turn() {
            Ok(Self {
                board: self.board.put(to, piece).remove(from),
                player_turn: self.turn().opponent(),
                en_passant: Option::None,
            })
        } else {
            Err(String::from("Can’t move pieces from the other player"))
        }
    }

    fn get_piece_at(&self, position: Position) -> Option<&Piece> {
        self.board.get(position)
    }

    pub fn list_moves(&self) -> Vector<Move> {
        self.board
            .iter()
            .filter(|(_, value)| value.player() == self.turn())
            .flat_map(|(key, value)| match value.piece_type() {
                PieceType::Pawn => pawn::list_pawn_moves(self, *key, value.player()),
                PieceType::Rook => rook::list_rook_moves(self, *key, value.player()),
                PieceType::Bishop => bishop::list_bishop_moves(self, *key, value.player()),
                PieceType::Queen => queen::list_queen_moves(self, *key, value.player()),
                PieceType::Knight => knight::list_knight_moves(self, *key, value.player()),
                PieceType::King => king::list_king_moves(self, *key, value.player()),
            })
            .collect()
    }

    fn create_move(&self, from: Position, to: Position) -> Move {
        Move::new(self, from, to)
    }

    fn create_move_en_passant(&self, from: Position, to: Position, en_passant: Position) -> Move {
        Move::new_with_en_passant(self, from, to, en_passant)
    }

    fn create_move_with_promotion(
        &self,
        from: Position,
        to: Position,
        promotion: PieceType,
    ) -> Move {
        Move::new_with_promotion(self, from, to, promotion)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Move<'a> {
    from: Position,
    to: Position,
    en_passant: Option<Position>,
    game: &'a Game,
    promotion: Option<PieceType>,
}

impl<'a> Move<'a> {
    fn new(game: &Game, from: Position, to: Position) -> Move<'_> {
        Move {
            from,
            to,
            en_passant: None,
            game,
            promotion: None,
        }
    }

    fn new_with_en_passant(
        game: &Game,
        from: Position,
        to: Position,
        en_passant: Position,
    ) -> Move<'_> {
        Move {
            from,
            to,
            en_passant: Some(en_passant),
            game,
            promotion: None,
        }
    }

    fn new_with_promotion(
        game: &Game,
        from: Position,
        to: Position,
        promotion: PieceType,
    ) -> Move<'_> {
        Move {
            from,
            to,
            en_passant: None,
            game,
            promotion: Some(promotion),
        }
    }

    fn new_game(&self) -> Game {
        let mut result = self
            .game
            .apply_move_with_en_passant(self.from, self.to, self.en_passant)
            .unwrap_or_else(|_| panic!("Invalid move {:?}", self));
        if Some(self.to) == self.game.en_passant {
            let position_to_remove =
                Position::from_chars(self.to.column() as char, self.from.row() as char).unwrap();
            result = Game {
                board: result.board.remove(position_to_remove),
                ..result
            };
        }
        result
    }

    fn promotion(&self) -> Option<PieceType> {
        self.promotion
    }
}

#[cfg(test)]
mod tests {
    use im::HashSet;

    use spectral::prelude::*;

    use board::Board;
    use board::Piece;
    use board::PieceType;
    use board::Player::*;
    use board::Position;
    use game::*;

    #[test]
    fn new_game() {
        // When
        let result = Game::new();

        // Then
        assert_eq!(result.board(), &Board::starting_position());
        assert_eq!(result.turn(), Player::White)
    }

    #[test]
    fn first_move() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game
            .execute_move(Position::from("e2").unwrap(), Position::from("e4").unwrap())
            .unwrap();

        // Then
        assert_eq!(
            game_after_move.get_piece_at(Position::from("e4").unwrap()),
            Some(&Piece::new(PieceType::Pawn, Player::White))
        );
        assert_eq!(
            game_after_move.get_piece_at(Position::from("e2").unwrap()),
            None
        );
        assert_eq!(game_after_move.turn(), Player::Black);
    }

    #[test]
    fn first_move_2() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game
            .execute_move(Position::from("d2").unwrap(), Position::from("d4").unwrap())
            .unwrap();

        // Then
        assert_eq!(
            game_after_move.get_piece_at(Position::from("d4").unwrap()),
            Some(&Piece::new(PieceType::Pawn, Player::White))
        );
    }

    #[test]
    fn second_move() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game
            .execute_move(Position::from("e2").unwrap(), Position::from("e4").unwrap())
            .and_then(|game| {
                game.execute_move(Position::from("e7").unwrap(), Position::from("e5").unwrap())
            });

        // Then
        assert_eq!(game_after_move.map(|game| game.turn()), Ok(Player::White));
    }

    #[test]
    fn do_not_move_opponent_pieces() {
        // Given
        let game = Game::new();

        // When
        let game_after_move =
            game.execute_move(Position::from("e7").unwrap(), Position::from("e5").unwrap());

        // Then
        assert!(game_after_move.is_err());
    }

    #[test]
    fn list_move_pawn_simple_white() {
        // Given
        let board = Board::empty().put(
            Position::from("e3").unwrap(),
            Piece::new(PieceType::Pawn, Player::White),
        );
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.list_moves();

        // Then
        assert_that!(result).equals_iterator(
            &[game.create_move(Position::from("e3").unwrap(), Position::from("e4").unwrap())]
                .iter(),
        )
    }

    #[test]
    fn list_move_pawn_simple_white2() {
        // Given
        let board = Board::empty().put(
            Position::from("h6").unwrap(),
            Piece::new(PieceType::Pawn, Player::White),
        );
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.list_moves();

        // Then
        assert_that!(result).equals_iterator(
            &[game.create_move(Position::from("h6").unwrap(), Position::from("h7").unwrap())]
                .iter(),
        )
    }

    #[test]
    fn list_move_pawn_simple_black() {
        // Given
        let board = Board::empty().put(
            Position::from("h6").unwrap(),
            Piece::new(PieceType::Pawn, Player::Black),
        );
        let game = Game::from_board(board, Player::Black);

        // When
        let result = game.list_moves();

        // Then
        assert_that!(result).equals_iterator(
            &[game.create_move(Position::from("h6").unwrap(), Position::from("h5").unwrap())]
                .iter(),
        )
    }

    #[test]
    fn list_move_only_current_player() {
        // Given
        let board = Board::empty().put(
            Position::from("h6").unwrap(),
            Piece::new(PieceType::Pawn, Player::Black),
        );
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.list_moves();

        // Then
        assert_that!(result.is_empty()).is_true();
    }

    #[test]
    fn list_move_pawn_starting_point_white() {
        // Given
        let board = Board::empty().put(
            Position::from("e2").unwrap(),
            Piece::new(PieceType::Pawn, Player::White),
        );
        let game = Game::from_board(board, Player::White);

        // When
        let result: Vector<Move> = game.list_moves();

        // Then
        assert_eq!(result.len(), 2);
        assert_that!(result).contains_all_of(&&[
            game.create_move(Position::from("e2").unwrap(), Position::from("e3").unwrap()),
            game.create_move_en_passant(
                Position::from("e2").unwrap(),
                Position::from("e4").unwrap(),
                Position::from("e3").unwrap(),
            ),
        ]);
    }

    #[test]
    fn list_move_pawn_starting_point_black_with_white() {
        // Given
        let board = Board::empty().put(
            Position::from("e7").unwrap(),
            Piece::new(PieceType::Pawn, Player::White),
        );
        let game = Game::from_board(board, Player::White);

        // When
        let result: Vector<Move> = game.list_moves();

        // Then
        assert_eq!(result.len(), 4);
        assert_that!(result).contains_all_of(&&[game.create_move_with_promotion(
            Position::from("e7").unwrap(),
            Position::from("e8").unwrap(),
            PieceType::Queen,
        )]);
    }

    #[test]
    fn list_move_pawn_starting_point_black() {
        // Given
        let board = Board::empty().put(
            Position::from("e7").unwrap(),
            Piece::new(PieceType::Pawn, Player::Black),
        );
        let game = Game::from_board(board, Player::Black);

        // When
        let result: Vector<Move> = game.list_moves();

        // Then
        assert_eq!(result.len(), 2);
        assert_that!(result).contains_all_of(&&[
            game.create_move(Position::from("e7").unwrap(), Position::from("e6").unwrap()),
            game.create_move_en_passant(
                Position::from("e7").unwrap(),
                Position::from("e5").unwrap(),
                Position::from("e6").unwrap(),
            ),
        ]);
    }

    #[test]
    fn list_move_pawn_capture_white() {
        // Given
        let board = Board::empty()
            .put(
                Position::from("e3").unwrap(),
                Piece::new(PieceType::Pawn, Player::White),
            )
            .put(
                Position::from("d4").unwrap(),
                Piece::new(PieceType::Pawn, Player::Black),
            );
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.list_moves();

        // Then
        assert_eq!(result.len(), 2);
        assert_that!(result).contains_all_of(&&[
            game.create_move(Position::from("e3").unwrap(), Position::from("e4").unwrap()),
            game.create_move(Position::from("e3").unwrap(), Position::from("d4").unwrap()),
        ]);
    }

    #[test]
    fn list_move_pawn_generate_en_passant() {
        // Given
        let board = Board::empty().put(
            Position::from("e2").unwrap(),
            Piece::new(PieceType::Pawn, Player::White),
        );
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.list_moves();

        // Then
        assert_eq!(result.len(), 2);
        assert_that!(result).contains_all_of(&&[
            Move::new(
                &game,
                Position::from("e2").unwrap(),
                Position::from("e3").unwrap(),
            ),
            Move::new_with_en_passant(
                &game,
                Position::from("e2").unwrap(),
                Position::from("e4").unwrap(),
                Position::from("e3").unwrap(),
            ),
        ]);
    }

    #[test]
    fn list_move_pawn_en_passant() {
        // Given
        let board = Board::empty()
            .put(
                Position::from("e4").unwrap(),
                Piece::new(PieceType::Pawn, Player::White),
            )
            .put(
                Position::from("d4").unwrap(),
                Piece::new(PieceType::Pawn, Player::Black),
            );
        let game = Game {
            board,
            player_turn: Player::White,
            en_passant: Some(Position::from("d5").unwrap()),
        };

        // When
        let result = game.list_moves();

        // Then
        assert_eq!(result.len(), 2);
        assert_that!(result).contains_all_of(&&[
            Move::new(
                &game,
                Position::from("e4").unwrap(),
                Position::from("e5").unwrap(),
            ),
            Move::new(
                &game,
                Position::from("e4").unwrap(),
                Position::from("d5").unwrap(),
            ),
        ]);
    }

    #[test]
    fn execute_move_pawn_en_passant() {
        // Given
        let board = Board::empty()
            .put(
                Position::from("e4").unwrap(),
                Piece::new(PieceType::Pawn, Player::White),
            )
            .put(
                Position::from("d4").unwrap(),
                Piece::new(PieceType::Pawn, Player::Black),
            );
        let game = Game {
            board,
            player_turn: Player::White,
            en_passant: Some(Position::from("d5").unwrap()),
        };

        // When
        let move_list = game.list_moves();
        let result = move_list
            .iter()
            .filter(|mv| {
                mv.from == Position::from("e4").unwrap() && mv.to == Position::from("d5").unwrap()
            })
            .next();

        // Then
        let new_board = Board::empty().put(
            Position::from("d5").unwrap(),
            Piece::new(PieceType::Pawn, Player::White),
        );
        let expected_new_game = Game {
            board: new_board,
            player_turn: Player::Black,
            en_passant: None,
        };
        assert_eq!(result.unwrap().new_game(), expected_new_game);
    }

    #[test]
    fn execute_legal_move() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game
            .execute_move(Position::from("e2").unwrap(), Position::from("e4").unwrap())
            .unwrap();

        // Then
        assert_eq!(
            game_after_move.get_piece_at(Position::from("e4").unwrap()),
            Some(&Piece::new(PieceType::Pawn, Player::White))
        );
        assert_eq!(
            game_after_move.get_piece_at(Position::from("e2").unwrap()),
            None
        );
        assert_eq!(game_after_move.turn(), Player::Black);
    }

    #[test]
    fn execute_illegal_move() {
        // Given
        let game = Game::new();

        // When
        let result =
            game.execute_move(Position::from("e2").unwrap(), Position::from("d3").unwrap());

        // Then
        assert_eq!(result, Err("Illegal move".to_string()));
    }

    #[test]
    fn execute_illegal_move2() {
        // Given
        let game = Game::new()
            .execute_move(Position::from("b2").unwrap(), Position::from("b3").unwrap())
            .unwrap()
            .execute_move(Position::from("b7").unwrap(), Position::from("b6").unwrap())
            .unwrap();

        // When
        let result =
            game.execute_move(Position::from("b1").unwrap(), Position::from("b2").unwrap());

        // Then
        assert_eq!(result, Err("Illegal move".to_string()));
    }

    #[test]
    fn rook() {
        // Given
        let board = Board::empty().put(
            Position::from("a1").unwrap(),
            Piece::new(PieceType::Rook, Player::White),
        );
        let game = Game::from_board(board, White);

        // When
        let result = game.list_moves();

        //Then
        let expected: HashSet<Position> = vector![
            "a2", "a3", "a4", "a5", "a6", "a7", "a8", "b1", "c1", "d1", "e1", "f1", "g1", "h1"
        ]
        .iter()
        .map(|pos| Position::from(pos).unwrap())
        .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn bishop() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("a1").unwrap(),
                Piece::new(PieceType::Bishop, Player::White),
            ),
            White,
        );

        // When
        let result = game.list_moves();

        //Then
        let expected: HashSet<Position> = vector!["b2", "c3", "d4", "e5", "f6", "g7", "h8"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn queen() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("a1").unwrap(),
                Piece::new(PieceType::Queen, Player::White),
            ),
            White,
        );

        // When
        let result = game.list_moves();

        //Then
        let expected: HashSet<Position> = [
            "a2", "a3", "a4", "a5", "a6", "a7", "a8", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
            "b2", "c3", "d4", "e5", "f6", "g7", "h8",
        ]
        .iter()
        .map(|pos| Position::from(pos).unwrap())
        .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn knight() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("a1").unwrap(),
                Piece::new(PieceType::Knight, Player::White),
            ),
            White,
        );

        // When
        let result = game.list_moves();

        //Then
        let expected: HashSet<Position> = ["b3", "c2"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn king() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("a1").unwrap(),
                Piece::new(PieceType::King, Player::White),
            ),
            White,
        );

        // When
        let result = game.list_moves();

        //Then
        let expected: HashSet<Position> = ["a2", "b1", "b2"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }
}
