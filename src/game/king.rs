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

pub fn list_castle_moves(game: &Game, position: Position, player: Player) -> Vector<Move<'_>> {
    let mut result = vector![];
    let ((castle_a, castle_h), row) = if player == Player::White {
        (game.castle_white, '1')
    } else {
        (game.castle_black, '8')
    };
    if position == Position::from_chars('e', row).unwrap()
        && castle_h
        && game
            .get_piece_at(Position::from_chars('g', row).unwrap())
            .is_none()
        && game
            .get_piece_at(Position::from_chars('f', row).unwrap())
            .is_none()
        && !game.is_check(Position::from_chars('f', row).unwrap())
    {
        result =
            result + vector![game.create_move(position, Position::from_chars('g', row).unwrap())];
    }
    if position == Position::from_chars('e', row).unwrap()
        && castle_a
        && game
            .get_piece_at(Position::from_chars('d', row).unwrap())
            .is_none()
        && game
            .get_piece_at(Position::from_chars('c', row).unwrap())
            .is_none()
        && !game.is_check(Position::from_chars('d', row).unwrap())
    {
        result =
            result + vector![game.create_move(position, Position::from_chars('c', row).unwrap())];
    }
    result
}

#[cfg(test)]
mod tests {
    use self::king::*;

    use im::HashSet;
    use spectral::boolean::BooleanAssertions;
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

    #[test]
    fn castle_h() {
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
        let result = list_castle_moves(&game, Position::from("e1").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["g1"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn castle_h_impossible() {
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
            false,
            false,
            false,
        );

        // When
        let result = list_castle_moves(&game, Position::from("e1").unwrap(), White);

        //Then
        assert_that!(result.is_empty()).is_true();
    }

    #[test]
    fn castle_a() {
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
        let result = list_castle_moves(&game, Position::from("e1").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["c1"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn castle_both() {
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
        let result = list_castle_moves(&game, Position::from("e1").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["c1", "g1"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn castle_h_intermediary_check() {
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
                )
                .put(
                    Position::from("f8").unwrap(),
                    Piece::new(PieceType::Rook, Player::Black),
                ),
            White,
            false,
            true,
            false,
            false,
        );

        // When
        let result = list_castle_moves(&game, Position::from("e1").unwrap(), White);

        //Then
        assert_that!(result.is_empty()).is_true();
    }

    #[test]
    fn castle_a_intermediary_check() {
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
                )
                .put(
                    Position::from("d8").unwrap(),
                    Piece::new(PieceType::Rook, Player::Black),
                ),
            White,
            true,
            false,
            false,
            false,
        );

        // When
        let result = list_castle_moves(&game, Position::from("e1").unwrap(), White);

        //Then
        assert_that!(result.is_empty()).is_true();
    }

    #[test]
    fn castle_h_black() {
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
        let result = list_castle_moves(&game, Position::from("e8").unwrap(), Black);

        //Then
        let expected: HashSet<Position> = ["g8"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }
}
