use board::*;
use game::*;

pub fn list_pawn_moves(game: &Game, key: Position, player: Player) -> Vector<Move<'_>> {
    let incr = |i| match player {
        Player::White => i + 1,
        Player::Black => i - 1,
    };
    let simple_move = Position::from_u8(key.column(), incr(key.row())).unwrap();
    let mut positions = vector![simple_move];
    let mut jump_position = None;
    if (game.turn() == Player::White && key.row() == b'2')
        || (game.turn() == Player::Black && key.row() == b'7')
    {
        jump_position = Some(Position::from_u8(key.column(), incr(incr(key.row()))).unwrap());
        positions.push_back(jump_position.unwrap());
    }
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
        .map(|position| {
            let en_passant = if Some(position) == jump_position {
                Some(simple_move)
            } else {
                None
            };
            game.create_move_en_passant(key, position, en_passant)
        })
        .collect()
}
