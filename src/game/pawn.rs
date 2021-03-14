use board::*;
use game::*;

pub fn list_pawn_moves(game: &Game, key: Position, player: Player) -> Vector<Move<'_>> {
    let incr = |i| match player {
        Player::White => i + 1,
        Player::Black => i - 1,
    };
    let mut jump_position = None;
    let simple_move = Position::from_u8(key.column(), incr(key.row()));
    let mut positions = if simple_move.and_then(|sm| game.board().get(sm)).is_none() {
        let mut advances: Vector<Position> = simple_move.into_iter().collect();
        if (game.turn() == Player::White && key.row() == b'2')
            || (game.turn() == Player::Black && key.row() == b'7')
        {
            let computed_jump = Position::from_u8(key.column(), incr(incr(key.row()))).unwrap();
            if game.board().get(computed_jump).is_none() {
                advances.push_back(computed_jump);
                jump_position = Some(computed_jump);
            };
        };
        advances
    } else {
        vector![]
    };
    let captures = vector![
        Position::from_u8(key.column() - 1, incr(key.row())),
        Position::from_u8(key.column() + 1, incr(key.row())),
    ]
    .into_iter()
    .flatten()
    .filter(|pos| {
        game.board()
            .get(*pos)
            .map_or(false, |piece| piece.player() == player.opponent())
            || game.en_passant == Some(*pos)
    })
    .collect();
    positions.append(captures);

    positions
        .into_iter()
        .flat_map(|position| {
            if Some(position) == jump_position {
                simple_move.map_or_else(
                    || vector![],
                    |sm| vector![game.create_move_en_passant(key, position, sm)],
                )
            } else if [b'1', b'8'].contains(&position.row()) {
                [
                    PieceType::Queen,
                    PieceType::Rook,
                    PieceType::Bishop,
                    PieceType::Knight,
                ]
                .iter()
                .map(|piece_type| game.create_move_with_promotion(key, position, *piece_type))
                .collect()
            } else {
                vector![game.create_move(key, position)]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use self::pawn::*;

    use spectral::prelude::ContainingIntoIterAssertions;
    use spectral::prelude::MappingIterAssertions;
    use spectral::*;

    use board::*;
    use game::*;

    #[test]
    fn promotion() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("a7").unwrap(),
                Piece::new(PieceType::Pawn, Player::White),
            ),
            Player::White,
        );

        // When
        let result = list_pawn_moves(&game, Position::from("a7").unwrap(), Player::White);

        // Then
        let expected = hashset![
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Knight
        ];
        let promotion_result = result.iter().map(|r| r.promotion.unwrap()).collect();
        assert_that!(promotion_result).is_equal_to(expected);
    }

    #[test]
    fn promotion2() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("b7").unwrap(),
                Piece::new(PieceType::Pawn, Player::White),
            ),
            Player::White,
        );

        // When
        let result = list_pawn_moves(&game, Position::from("b7").unwrap(), Player::White);

        // Then
        let expected = hashset![
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Knight
        ];
        let promotion_result = result.iter().map(|r| r.promotion.unwrap()).collect();
        assert_that!(promotion_result).is_equal_to(expected);
    }

    #[test]
    fn no_promotion() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("a6").unwrap(),
                Piece::new(PieceType::Pawn, Player::White),
            ),
            Player::White,
        );

        // When
        let result = list_pawn_moves(&game, Position::from("a6").unwrap(), Player::White);

        // Then
        assert_that!(result).mapped_contains(|x| x.promotion, &None);
    }

    #[test]
    fn promotion_black() {
        // Given
        let game = Game::from_board(
            Board::empty().put(
                Position::from("a2").unwrap(),
                Piece::new(PieceType::Pawn, Player::Black),
            ),
            Player::Black,
        );

        // When
        let result = list_pawn_moves(&game, Position::from("a2").unwrap(), Player::Black);

        // Then
        let expected = hashset![
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Knight
        ];
        let promotion_result = result.iter().map(|r| r.promotion.unwrap()).collect();
        assert_that!(promotion_result).is_equal_to(expected);
    }

    #[test]
    fn no_capture() {
        // Given
        let game = Game::from_board(
            Board::empty()
                .put(
                    Position::from("e6").unwrap(),
                    Piece::new(PieceType::Pawn, Player::White),
                )
                .put(
                    Position::from("e7").unwrap(),
                    Piece::new(PieceType::Pawn, Player::Black),
                ),
            Player::Black,
        );

        // When
        let result = list_pawn_moves(&game, Position::from("e7").unwrap(), Player::Black);

        // Then
        assert_that!(result.iter().map(|mv| mv.to).collect::<Vec<Position>>())
            .does_not_contain(&Position::from("e6").unwrap())
    }

    #[test]
    fn no_capture_on_start() {
        // Given
        let game = Game::from_board(
            Board::empty()
                .put(
                    Position::from("e5").unwrap(),
                    Piece::new(PieceType::Pawn, Player::White),
                )
                .put(
                    Position::from("e7").unwrap(),
                    Piece::new(PieceType::Pawn, Player::Black),
                ),
            Player::Black,
        );

        // When
        let result = list_pawn_moves(&game, Position::from("e7").unwrap(), Player::Black);

        // Then
        assert_that!(result.iter().map(|mv| mv.to).collect::<Vec<Position>>())
            .does_not_contain(&Position::from("e5").unwrap())
    }
}
