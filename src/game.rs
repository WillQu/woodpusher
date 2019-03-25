use board::Board;
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

    fn apply_move(&self, from: &str, to: &str) -> Game {
        self.get_piece_at(from)
            .map_or_else(
                || self.clone(),
                |piece| Game{
                    board: self.board.put(to, *piece).remove(from),
                    player_turn: self.turn().opponent(),
                }
            )
    }

    fn get_piece_at(&self, position: &str) -> Option<&Piece> {
        self.board.get(position)
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use board::PieceType;

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
        let game_after_move = game.apply_move("e2", "e4");

        // Then
        assert_eq!(game_after_move.get_piece_at("e4"), Some(&Piece::new(PieceType::Pawn, Player::White)));
        assert_eq!(game_after_move.get_piece_at("e2"), None);
        assert_eq!(game_after_move.turn(), Player::Black);
    }

    #[test]
    fn first_move_2() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game.apply_move("d2", "d4");

        // Then
        assert_eq!(game_after_move.get_piece_at("d4"), Some(&Piece::new(PieceType::Pawn, Player::White)));
    }

    #[test]
    fn first_move_3() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game.apply_move("g1", "f3");

        // Then
        assert_eq!(game_after_move.get_piece_at("f3"), Some(&Piece::new(PieceType::Knight, Player::White)));
    }

    #[test]
    fn second_move() {
        // Given
        let game = Game::new();

        // When
        let game_after_move = game.apply_move("e2", "e4").apply_move("e7", "e5");

        // Then
        assert_eq!(game_after_move.turn(), Player::White);
    }
}
