use board::Board;
use board::Position;
use board::Player;
use board::Piece;

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

    fn board(&self) -> &Board {
        &self.board
    }

    fn turn(&self) -> Player {
        self.player_turn
    }

    fn apply_move(&self, from: &Position, to: Position) -> Game {
        self.get_piece_at(from)
            .map_or_else(
                || self.clone(),
                |piece| Game{
                    board: self.board.put(to, *piece).remove(from),
                    player_turn: self.turn().opponent(),
                }
            )
    }

    fn get_piece_at(&self, position: &Position) -> Option<&Piece> {
        self.board.get(position)
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use board::PieceType;
    use board::Position;

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
        let game_after_move = game.apply_move(&Position::from("e2").unwrap(), Position::from("e4").unwrap());

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
        let game_after_move = game.apply_move(&Position::from("d2").unwrap(), Position::from("d4").unwrap());

        // Then
        assert_eq!(game_after_move.get_piece_at(&Position::from("d4").unwrap()), Some(&Piece::new(PieceType::Pawn, Player::White)));
    }

    #[test]
    fn first_move_3() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game.apply_move(&Position::from("g1").unwrap(), Position::from("f3").unwrap());

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
            .apply_move(&Position::from("e7").unwrap(), Position::from("e5").unwrap());

        // Then
        assert_eq!(game_after_move.turn(), Player::White);
    }
}
