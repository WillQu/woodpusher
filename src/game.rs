use board::Board;
use board::Position;
use board::Player;
use board::Piece;
use im::Vector;

#[derive(Clone)]
struct Game {
    board: Board,
    player_turn: Player,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::starting_position(),
            player_turn: Player::White,
        }
    }

    fn from_board(board: Board, player: Player) -> Game {
        Game {
            board: board,
            player_turn: player,
        }
    }

    fn board(&self) -> &Board {
        &self.board
    }

    fn turn(&self) -> Player {
        self.player_turn
    }

    fn apply_move(&self, from: &Position, to: Position) -> Result<Game, String> {
        self.get_piece_at(from)
            .map_or_else(
                || Err(format!("No piece at {}", from)),
                |piece| self.apply_move_to_piece(from, to, piece)
            )
    }

    fn apply_move_to_piece(&self, from: &Position, to: Position, piece: &Piece) -> Result<Game, String> {
        if piece.player() == self.turn() {
            Ok(Game {
                board: self.board.put(to, *piece).remove(from),
                player_turn: self.turn().opponent(),
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
            .map(|(key, value)| self.list_pawn_moves(key, value))
            .collect()
    }

    fn create_move(&self, from: Position, to: Position) -> Move {
        Move{from: from, to: to}
    }

    fn list_pawn_moves(&self, key: &Position, value: &Piece) -> Move {
        let incr = |i| match value.player() {
            Player::White => i+1,
            Player::Black => i-1,
        };
        self.create_move(*key, Position::new(key.column(), incr(key.row() as u8) as char).unwrap())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Move {
    from: Position,
    to: Position,
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
}
