use board::Board;
use board::Position;
use board::Player;
use board::Piece;
use im::Vector;

#[derive(Clone, Debug, PartialEq)]
struct Game {
    board: Board,
    player_turn: Player,
    en_passant: Option<Position>,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::starting_position(),
            player_turn: Player::White,
            en_passant: Option::None,
        }
    }

    fn from_board(board: Board, player: Player) -> Game {
        Game {
            board: board,
            player_turn: player,
            en_passant: Option::None,
        }
    }

    fn board(&self) -> &Board {
        &self.board
    }

    fn turn(&self) -> Player {
        self.player_turn
    }

    fn apply_move(&self, from: &Position, to: Position) -> Result<Game, String> {
        self.apply_move_with_en_passant(from, to, None)
    }

    fn apply_move_with_en_passant(&self, from: &Position, to: Position, en_passant: Option<Position>) -> Result<Game, String> {
        self.get_piece_at(from)
            .map_or_else(
                || Err(format!("No piece at {}", from)),
                |piece| self
                    .apply_move_to_piece(from, to, piece)
                    .map(|game|
                        Game {
                            en_passant: en_passant,
                            .. game
                    })
            )
    }

    fn apply_move_to_piece(&self, from: &Position, to: Position, piece: &Piece) -> Result<Game, String> {
        if piece.player() == self.turn() {
            Ok(Game {
                board: self.board.put(to, *piece).remove(from),
                player_turn: self.turn().opponent(),
                en_passant: Option::None,
            })
        } else {
            Err(String::from("Can’t move pieces from the other player"))
        }
    }

    fn get_piece_at(&self, position: &Position) -> Option<&Piece> {
        self.board.get(position)
    }

    fn list_moves(&self) -> Vector<Move> {
        self
            .board
            .iter()
            .filter(|(_, value)| value.player() == self.turn())
            .flat_map(|(key, value)| self.list_pawn_moves(key, value))
            .collect()
    }

    fn create_move(&self, from: Position, to: Position) -> Move {
        self.create_move_en_passant(from, to, None)
    }

    fn create_move_en_passant(&self, from: Position, to: Position, en_passant: Option<Position>) -> Move {
        Move{from: from, to: to, en_passant: en_passant, game: self}
    }

    fn list_pawn_moves(&self, key: &Position, value: &Piece) -> Vector<Move> {
        let incr = |i| match value.player() {
            Player::White => i+1,
            Player::Black => i-1,
        };
        let simple_move = Position::from_u8(key.column(), incr(key.row())).unwrap();
        let mut positions = vector![simple_move];
        let mut jump_position = None;
        if (self.turn() == Player::White && key.row() == '2' as u8) || (self.turn() == Player::Black && key.row() == '7' as u8) {
            jump_position = Some(Position::from_u8(key.column(), incr(incr(key.row()))).unwrap());
            positions.push_back(jump_position.unwrap());
        }
        let captures = vector![
            Position::from_u8(key.column() - 1, incr(key.row())),
            Position::from_u8(key.column() + 1, incr(key.row())),]
            .into_iter()
            .flatten()
            .filter(|pos| self
                .board()
                .get(pos)
                .map_or(false, |piece| piece.player() == value.player().opponent()))
            .collect();
        positions.append(captures);

        positions
            .into_iter()
            .map(|position| {
                let en_passant = if Some(position) == jump_position {
                    Some(simple_move)
                } else {
                    None
                };
                self.create_move_en_passant(*key, position, en_passant)
            })
            .collect()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Move<'a> {
    from: Position,
    to: Position,
    en_passant: Option<Position>,
    game: &'a Game,
}

impl<'a> Move<'a> {
    fn execute(&self) -> Game {
        self.game.apply_move_with_en_passant(&self.from, self.to, self.en_passant).expect(&format!("Invalid move {:?}", self))
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use game::*;
    use board::PieceType;
    use board::Player;
    use board::Position;
    use board::Board;
    use board::Piece;

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
        let game_after_move = game.apply_move(&Position::from("e2").unwrap(), Position::from("e4").unwrap()).unwrap();

        // Then
        assert_eq!(game_after_move.get_piece_at(&Position::from("e4").unwrap()), Some(&Piece::new(PieceType::Pawn, Player::White)));
        assert_eq!(game_after_move.get_piece_at(&Position::from("e2").unwrap()), None);
        assert_eq!(game_after_move.turn(), Player::Black);
    }

    #[test]
    fn first_move_2() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game.apply_move(&Position::from("d2").unwrap(), Position::from("d4").unwrap()).unwrap();

        // Then
        assert_eq!(game_after_move.get_piece_at(&Position::from("d4").unwrap()), Some(&Piece::new(PieceType::Pawn, Player::White)));
    }

    #[test]
    fn first_move_3() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game.apply_move(&Position::from("g1").unwrap(), Position::from("f3").unwrap()).unwrap();

        // Then
        assert_eq!(game_after_move.get_piece_at(&Position::from("f3").unwrap()), Some(&Piece::new(PieceType::Knight, Player::White)));
    }

    #[test]
    fn second_move() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game
            .apply_move(&Position::from("e2").unwrap(), Position::from("e4").unwrap())
            .and_then(|game| game.apply_move(&Position::from("e7").unwrap(), Position::from("e5").unwrap()));

        // Then
        assert_eq!(game_after_move.map(|game| game.turn()), Ok(Player::White));
    }

    #[test]
    fn do_not_move_opponent_pieces() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game
            .apply_move(&Position::from("e7").unwrap(), Position::from("e5").unwrap());

        // Then
        assert!(game_after_move.is_err());
    }

    #[test]
    fn list_move_pawn_simple_white() {
        // Given
        let board = Board::empty().put(Position::from("e3").unwrap(), Piece::new(PieceType::Pawn, Player::White));
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.list_moves();

        // Then
        assert_that!(result).equals_iterator(&[game.create_move(Position::from("e3").unwrap(), Position::from("e4").unwrap())].iter())
    }

    #[test]
    fn list_move_pawn_simple_white2() {
        // Given
        let board = Board::empty().put(Position::from("h6").unwrap(), Piece::new(PieceType::Pawn, Player::White));
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.list_moves();

        // Then
        assert_that!(result).equals_iterator(&[game.create_move(Position::from("h6").unwrap(), Position::from("h7").unwrap())].iter())
    }

    #[test]
    fn list_move_pawn_simple_black() {
        // Given
        let board = Board::empty().put(Position::from("h6").unwrap(), Piece::new(PieceType::Pawn, Player::Black));
        let game = Game::from_board(board, Player::Black);

        // When
        let result = game.list_moves();

        // Then
        assert_that!(result).equals_iterator(&[game.create_move(Position::from("h6").unwrap(), Position::from("h5").unwrap())].iter())
    }

    #[test]
    fn list_move_only_current_player() {
        // Given
        let board = Board::empty().put(Position::from("h6").unwrap(), Piece::new(PieceType::Pawn, Player::Black));
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.list_moves();

        // Then
        assert_that!(result.is_empty()).is_true();
    }

    #[test]
    fn list_move_pawn_starting_point_white() {
        // Given
        let board = Board::empty().put(Position::from("e2").unwrap(), Piece::new(PieceType::Pawn, Player::White));
        let game = Game::from_board(board, Player::White);

        // When
        let result: Vector<Move> = game.list_moves();

        // Then
        assert_eq!(result.len(), 2);
        assert_that!(result).contains_all_of(&&[
            game.create_move(Position::from("e2").unwrap(), Position::from("e3").unwrap()),
            game.create_move_en_passant(Position::from("e2").unwrap(), Position::from("e4").unwrap(), Some(Position::from("e3").unwrap())),
        ]);
    }

    #[test]
    fn list_move_pawn_starting_point_black_with_white() {
        // Given
        let board = Board::empty().put(Position::from("e7").unwrap(), Piece::new(PieceType::Pawn, Player::White));
        let game = Game::from_board(board, Player::White);

        // When
        let result: Vector<Move> = game.list_moves();

        // Then
        assert_eq!(result.len(), 1);
        assert_that!(result).contains_all_of(&&[
            game.create_move(Position::from("e7").unwrap(), Position::from("e8").unwrap()),
        ]);
    }

    #[test]
    fn list_move_pawn_starting_point_black() {
        // Given
        let board = Board::empty().put(Position::from("e7").unwrap(), Piece::new(PieceType::Pawn, Player::Black));
        let game = Game::from_board(board, Player::Black);

        // When
        let result: Vector<Move> = game.list_moves();

        // Then
        assert_eq!(result.len(), 2);
        assert_that!(result).contains_all_of(&&[
            game.create_move(Position::from("e7").unwrap(), Position::from("e6").unwrap()),
            game.create_move_en_passant(Position::from("e7").unwrap(), Position::from("e5").unwrap(), Some(Position::from("e6").unwrap())),
        ]);
    }

    #[test]
    fn list_move_pawn_capture_white() {
        // Given
        let board = Board::empty()
            .put(Position::from("e3").unwrap(), Piece::new(PieceType::Pawn, Player::White))
            .put(Position::from("d4").unwrap(), Piece::new(PieceType::Pawn, Player::Black));
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
        let board = Board::empty()
            .put(Position::from("e2").unwrap(), Piece::new(PieceType::Pawn, Player::White));
        let game = Game::from_board(board, Player::White);

        // When
        let result = game.list_moves();

        // Then
        assert_eq!(result.len(), 2);
        assert_that!(result).contains_all_of(&&[
            Move {from: Position::from("e2").unwrap(), to: Position::from("e3").unwrap(), en_passant: None, game: &game},
            Move {from: Position::from("e2").unwrap(), to: Position::from("e4").unwrap(), en_passant: Some(Position::from("e3").unwrap()), game: &game},
        ]);
    }
}
