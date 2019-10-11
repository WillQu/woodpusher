use board::*;
use game::*;

pub fn list_king_moves(game: &Game, position: Position, player: Player) -> Vector<Move<'_>> {
    let base_list = move_list::generate_moves_one_square(
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
    );
	if position == Position::from("e1").unwrap() && game.castle_h {
		base_list + vector![game.create_move(position, Position::from("g1").unwrap())]
	} else if position == Position::from("e1").unwrap() && game.castle_a {
		base_list + vector![game.create_move(position, Position::from("c1").unwrap())]
	} else {
		base_list
	}
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

    #[test]
    fn castle_h() {
        // Given
        let game = Game::from_board(
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
        );

        // When
        let result = list_king_moves(&game, Position::from("e1").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["d1", "d2", "e2", "f2", "f1", "g1"]
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
        );

        // When
        let result = list_king_moves(&game, Position::from("e1").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["d1", "d2", "e2", "f2", "f1"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn castle_a() {
        // Given
        let game = Game::from_board_with_castle (
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
        );

        // When
        let result = list_king_moves(&game, Position::from("e1").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["d1", "d2", "e2", "f2", "f1", "c1"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }

    #[test]
    fn castle_both() {
        // Given
        let game = Game::from_board_with_castle (
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
        );

        // When
        let result = list_king_moves(&game, Position::from("e1").unwrap(), White);

        //Then
        let expected: HashSet<Position> = ["d1", "d2", "e2", "f2", "f1", "c1", "g1"]
            .iter()
            .map(|pos| Position::from(pos).unwrap())
            .collect();
        let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
        assert_that!(result_positions).is_equal_to(expected);
    }
}
