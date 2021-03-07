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
    castle_white: (bool, bool),
    castle_black: (bool, bool),
}

impl Game {
    pub fn new() -> Self {
        Self::from_board(Board::starting_position(), Player::White)
    }

    pub fn from_board(board: Board, player: Player) -> Self {
        Self::from_board_with_castle(board, player, true, true, true, true)
    }

    pub fn from_board_with_castle(
        board: Board,
        player: Player,
        castle_a_white: bool,
        castle_h_white: bool,
        castle_a_black: bool,
        castle_h_black: bool,
    ) -> Self {
        Self {
            board,
            player_turn: player,
            en_passant: Option::None,
            castle_white: (castle_a_white, castle_h_white),
            castle_black: (castle_a_black, castle_h_black),
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn turn(&self) -> Player {
        self.player_turn
    }

    pub fn execute_move(&self, from: Position, to: Position) -> Result<Self, String> {
        self.execute_promotion(from, to, None)
    }

    pub fn execute_promotion(
        &self,
        from: Position,
        to: Position,
        promotion: Option<PieceType>,
    ) -> Result<Self, String> {
        self.list_moves()
            .iter()
            .find(|mv| mv.from == from && mv.to == to && mv.promotion == promotion)
            .map(Move::new_game)
            .ok_or_else(|| "Illegal move".to_string())
    }

    fn apply_move_with_en_passant(
        &self,
        from: Position,
        to: Position,
        en_passant: Option<Position>,
        promotion: Option<PieceType>,
    ) -> Result<Self, String> {
        self.get_piece_at(from).map_or_else(
            || Err(format!("No piece at {}", from)),
            |piece| {
                self.apply_promotion_to_piece(from, to, *piece, promotion)
                    .map(|game| Self { en_passant, ..game })
            },
        )
    }

    fn apply_promotion_to_piece(
        &self,
        from: Position,
        to: Position,
        piece: Piece,
        promotion: Option<PieceType>,
    ) -> Result<Self, String> {
        if piece.player() == self.turn() {
            let new_piece =
                promotion.map_or(piece, |piece_type| Piece::new(piece_type, piece.player()));
            Ok(Self {
                board: self.board.put(to, new_piece).remove(from),
                player_turn: self.turn().opponent(),
                en_passant: Option::None,
                castle_white: self.castle_white,
                castle_black: self.castle_black,
            })
        } else {
            Err(String::from("Can’t move pieces from the other player"))
        }
    }

    fn get_piece_at(&self, position: Position) -> Option<&Piece> {
        self.board.get(position)
    }

    pub fn list_moves(&self) -> Vector<Move> {
        let king_position = self
            .board
            .iter()
            .find(|(_, piece)| {
                piece.piece_type() == PieceType::King && piece.player() == self.player_turn
            })
            .map(|(position, _)| position);
        let castles = king_position.map_or(Vector::new(), |position| {
            king::list_castle_moves(self, *position, self.player_turn)
        });
        (self.list_moves_no_check() + castles)
            .into_iter()
            .filter(|mov| {
                !Game {
                    player_turn: self.player_turn,
                    ..mov.new_game()
                }
                .is_king_check()
            })
            .collect()
    }

    fn list_moves_no_check(&self) -> Vector<Move> {
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

    fn is_check(&self, position: Position) -> bool {
        Game {
            player_turn: self.player_turn.opponent(),
            ..self.clone()
        }
        .list_moves_no_check()
        .into_iter()
        .any(|mv| mv.to == position)
    }

    fn is_king_check(&self) -> bool {
        self.board
            .iter()
            .find(|(_, piece)| {
                piece.piece_type() == PieceType::King && piece.player() == self.player_turn
            })
            .map_or(false, |(position, _)| self.is_check(*position))
    }

    pub fn is_stalemate(&self) -> bool {
        self.list_moves().is_empty() && !self.is_king_check()
    }

    pub fn is_mate(&self) -> bool {
        self.list_moves().is_empty() && self.is_king_check()
    }

    fn disable_castle(&self, player: Player) -> Game {
        match player {
            Player::White => Game {
                castle_white: (false, false),
                ..self.clone()
            },
            Player::Black => Game {
                castle_black: (false, false),
                ..self.clone()
            },
        }
    }

    pub fn list_pieces(&self) -> Vector<Piece> {
        self.board.iter().map(|(_, piece)| *piece).collect()
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

    pub fn new_game(&self) -> Game {
        let mut result = self
            .game
            .apply_move_with_en_passant(self.from, self.to, self.en_passant, self.promotion)
            .unwrap_or_else(|_| panic!("Invalid move {:?}", self));
        if Some(self.to) == self.game.en_passant {
            let position_to_remove =
                Position::from_chars(self.to.column() as char, self.from.row() as char).unwrap();
            result = Game {
                board: result.board.remove(position_to_remove),
                ..result
            };
        }
        self.finalize_castle(result)
    }

    fn detect_castle(&self) -> bool {
        let piece = self
            .game
            .board
            .get(self.from)
            .expect("No piece at \"from\" position");
        piece.piece_type() == PieceType::King
            && (self.to.column() == ('g' as u8) || self.to.column() == ('c' as u8))
    }

    fn finalize_castle(&self, game: Game) -> Game {
        if self.detect_castle() {
            let (rook_from, rook_to) = if self.to.column() == ('c' as u8) {
                (b'a', b'd')
            } else {
                (b'h', b'f')
            };
            Game {
                board: game
                    .board
                    .remove(Position::from_u8(rook_from, self.to.row()).unwrap())
                    .put(
                        Position::from_u8(rook_to, self.to.row()).unwrap(),
                        Piece::new(PieceType::Rook, game.player_turn.opponent()),
                    ),
                ..game.disable_castle(game.player_turn.opponent())
            }
        } else if self.from == Position::from("h1").unwrap() {
            Game {
                castle_white: (game.castle_white.0, false),
                ..game
            }
        } else if self.from == Position::from("a1").unwrap() {
            Game {
                castle_white: (false, game.castle_white.1),
                ..game
            }
        } else if self.from == Position::from("h8").unwrap() {
            Game {
                castle_black: (game.castle_black.0, false),
                ..game
            }
        } else if self.from == Position::from("a8").unwrap() {
            Game {
                castle_black: (false, game.castle_black.1),
                ..game
            }
        } else {
            game
        }
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
        let result = game.list_moves_no_check();

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
        let result = game.list_moves_no_check();

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
        let result = game.list_moves_no_check();

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
        let result = game.list_moves_no_check();

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
        let result: Vector<Move> = game.list_moves_no_check();

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
        let result: Vector<Move> = game.list_moves_no_check();

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
        let result: Vector<Move> = game.list_moves_no_check();

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
        let result = game.list_moves_no_check();

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
        let result = game.list_moves_no_check();

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
            castle_white: (true, true),
            castle_black: (true, true),
        };

        // When
        let result = game.list_moves_no_check();

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
            castle_white: (true, true),
            castle_black: (true, true),
        };

        // When
        let move_list = game.list_moves_no_check();
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
            castle_white: (true, true),
            castle_black: (true, true),
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
        let result = game.list_moves_no_check();

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
        let result = game.list_moves_no_check();

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
        let result = game.list_moves_no_check();

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
        let result = game.list_moves_no_check();

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
        let result = game.list_moves_no_check();

        //Then
        let expected: HashSet<Position> = ["a2", "b1", "b2"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn execute_promotion() {
        // Given
        let board = Board::empty().put(
            Position::from("e7").unwrap(),
            Piece::new(PieceType::Pawn, Player::White),
        );
        let game = Game {
            board,
            player_turn: Player::White,
            en_passant: None,
            castle_white: (true, true),
            castle_black: (true, true),
        };

        // When
        let result = game.execute_promotion(
            Position::from("e7").unwrap(),
            Position::from("e8").unwrap(),
            Some(PieceType::Queen),
        );

        // Then
        let new_board = Board::empty().put(
            Position::from("e8").unwrap(),
            Piece::new(PieceType::Queen, Player::White),
        );
        let expected_new_game = Game {
            board: new_board,
            player_turn: Player::Black,
            en_passant: None,
            castle_white: (true, true),
            castle_black: (true, true),
        };
        assert_eq!(result.unwrap(), expected_new_game);
    }

    #[test]
    fn is_black_check() {
        // Given
        let board = Board::empty()
            .put(
                Position::from("e3").unwrap(),
                Piece::new(PieceType::Pawn, Player::White),
            )
            .put(
                Position::from("d4").unwrap(),
                Piece::new(PieceType::King, Player::Black),
            );
        let game = Game::from_board(board, Player::Black);

        // When
        let result = game.is_check(Position::from("d4").unwrap());

        // Then
        assert_that!(result).is_true();
    }

    #[test]
    fn is_not_check() {
        // Given
        let board = Board::empty()
            .put(
                Position::from("e3").unwrap(),
                Piece::new(PieceType::Pawn, Player::White),
            )
            .put(
                Position::from("d5").unwrap(),
                Piece::new(PieceType::King, Player::Black),
            );
        let game = Game::from_board(board, Player::Black);

        // When
        let result = game.is_check(Position::from("d5").unwrap());

        // Then
        assert_that!(result).is_false();
    }

    #[test]
    fn is_white_check() {
        // Given
        let board = Board::empty()
            .put(
                Position::from("e3").unwrap(),
                Piece::new(PieceType::Pawn, Player::Black),
            )
            .put(
                Position::from("d2").unwrap(),
                Piece::new(PieceType::King, Player::White),
            );
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.is_check(Position::from("d2").unwrap());

        // Then
        assert_that!(result).is_true();
    }

    #[test]
    fn dont_list_check_moves() {
        // Given
        let board = Board::empty()
            .put(
                Position::from("d4").unwrap(),
                Piece::new(PieceType::King, Player::Black),
            )
            .put(
                Position::from("d2").unwrap(),
                Piece::new(PieceType::King, Player::White),
            );
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.list_moves();

        //Then
        let expected: HashSet<Position> = ["c2", "e2", "c1", "d1", "e1"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn castle() {
        // Given
        let game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("h1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            White,
            false,
            true,
            false,
            false,
        );

        // When
        let result = game.list_moves();

        //Then
        assert_that!(result).matching_contains(|mv| {
            mv.from == Position::from("e1").unwrap() && mv.to == Position::from("g1").unwrap()
        });
    }

    #[test]
    fn execute_castle() {
        // Given
        let game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("h1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            White,
            false,
            true,
            false,
            false,
        );

        // When
        let result =
            game.execute_move(Position::from("e1").unwrap(), Position::from("g1").unwrap());

        //Then
        let expected_game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("g1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("f1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            Black,
            false,
            false,
            false,
            false,
        );
        assert_eq!(result, Ok(expected_game));
    }

    #[test]
    fn execute_castle_a() {
        // Given
        let game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("a1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            White,
            true,
            false,
            false,
            false,
        );

        // When
        let result =
            game.execute_move(Position::from("e1").unwrap(), Position::from("c1").unwrap());

        //Then
        let expected_game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("c1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("d1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            Black,
            false,
            false,
            false,
            false,
        );
        assert_eq!(result, Ok(expected_game));
    }

    #[test]
    fn execute_castle_black_h() {
        // Given
        let game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("h8").unwrap(),
                    Piece::new(PieceType::Rook, Player::Black),
                ),
            Black,
            true,
            true,
            false,
            true,
        );

        // When
        let result =
            game.execute_move(Position::from("e8").unwrap(), Position::from("g8").unwrap());

        //Then
        let expected_game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("g8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("f8").unwrap(),
                    Piece::new(PieceType::Rook, Player::Black),
                ),
            White,
            true,
            true,
            false,
            false,
        );
        assert_eq!(result, Ok(expected_game));
    }

    #[test]
    fn disable_castle_on_rook_move() {
        // Given
        let game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("h1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            White,
            false,
            true,
            false,
            false,
        );

        // When
        let result =
            game.execute_move(Position::from("h1").unwrap(), Position::from("g1").unwrap());

        //Then
        let expected_game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("g1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            Black,
            false,
            false,
            false,
            false,
        );
        assert_eq!(result, Ok(expected_game));
    }

    #[test]
    fn disable_castle_on_rook_move2() {
        // Given
        let game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("h1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            White,
            true,
            true,
            false,
            false,
        );

        // When
        let result =
            game.execute_move(Position::from("h1").unwrap(), Position::from("g1").unwrap());

        //Then
        let expected_game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("g1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            Black,
            true,
            false,
            false,
            false,
        );
        assert_eq!(result, Ok(expected_game));
    }

    #[test]
    fn disable_castle_on_rook_move_a() {
        // Given
        let game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("a1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            White,
            true,
            true,
            false,
            false,
        );

        // When
        let result =
            game.execute_move(Position::from("a1").unwrap(), Position::from("b1").unwrap());

        //Then
        let expected_game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("b1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            Black,
            false,
            true,
            false,
            false,
        );
        assert_eq!(result, Ok(expected_game));
    }

    #[test]
    fn disable_castle_on_rook_move_a_2() {
        // Given
        let game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("a1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            White,
            true,
            false,
            false,
            false,
        );

        // When
        let result =
            game.execute_move(Position::from("a1").unwrap(), Position::from("b1").unwrap());

        //Then
        let expected_game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("b1").unwrap(),
                    Piece::new(PieceType::Rook, Player::White),
                ),
            Black,
            false,
            false,
            false,
            false,
        );
        assert_eq!(result, Ok(expected_game));
    }

    #[test]
    fn disable_castle_on_rook_move_black() {
        // Given
        let game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("h8").unwrap(),
                    Piece::new(PieceType::Rook, Player::Black),
                ),
            Black,
            false,
            false,
            false,
            true,
        );

        // When
        let result =
            game.execute_move(Position::from("h8").unwrap(), Position::from("g8").unwrap());

        //Then
        let expected_game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("g8").unwrap(),
                    Piece::new(PieceType::Rook, Player::Black),
                ),
            White,
            false,
            false,
            false,
            false,
        );
        assert_eq!(result, Ok(expected_game));
    }

    #[test]
    fn disable_castle_on_rook_move_a_black() {
        // Given
        let game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("a8").unwrap(),
                    Piece::new(PieceType::Rook, Player::Black),
                ),
            Black,
            false,
            false,
            true,
            true,
        );

        // When
        let result =
            game.execute_move(Position::from("a8").unwrap(), Position::from("b8").unwrap());

        //Then
        let expected_game = Game::from_board_with_castle(
            Board::empty()
                .put(
                    Position::from("e1").unwrap(),
                    Piece::new(PieceType::King, Player::White),
                )
                .put(
                    Position::from("e8").unwrap(),
                    Piece::new(PieceType::King, Player::Black),
                )
                .put(
                    Position::from("b8").unwrap(),
                    Piece::new(PieceType::Rook, Player::Black),
                ),
            White,
            false,
            false,
            false,
            true,
        );
        assert_eq!(result, Ok(expected_game));
    }
}
