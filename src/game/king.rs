use board::*;
use game::*;

pub fn list_king_moves(game: &Game, position: Position, player: Player) -> Vector<Move<'_>> {
    move_list::generate_moves_one_square(
        game,
        player,
        position,
        &[
            (0, 1),
            (1, 0),
            (1, 1),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ],
    )
}

#[cfg(test)]
mod tests {
    use self::king::*;

    use im::HashSet;
    use spectral::*;

    use board::Player::*;
    use board::*;
    use game::*;

    #[test]
    fn from_a1() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("a1").unwrap(),
                Piece::new(PieceType::King, Player::White),
            ),
            White,
        );

        // When
        let result = list_king_moves(&game, Position::from("a1").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["a2", "b1", "b2"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn from_a8() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("a8").unwrap(),
                Piece::new(PieceType::King, Player::White),
            ),
            White,
        );

        // When
        let result = list_king_moves(&game, Position::from("a8").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["a7", "b8", "b7"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn from_h8() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("h8").unwrap(),
                Piece::new(PieceType::King, Player::White),
            ),
            White,
        );

        // When
        let result = list_king_moves(&game, Position::from("h8").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["h7", "g8", "g7"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn from_h1() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("h1").unwrap(),
                Piece::new(PieceType::King, Player::White),
            ),
            White,
        );

        // When
        let result = list_king_moves(&game, Position::from("h1").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["h2", "g1", "g2"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }
}
