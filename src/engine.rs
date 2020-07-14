use im::Vector;

use board::Piece;
use board::PieceType;
use board::Player;
use game::Game;
use game::Move;

pub fn select_move<'a>(game: &'a Game) -> Vector<Move<'a>> {
    let mut result: Vec<Move<'a>> = game.list_moves().into_iter().collect();
    if game.turn() == Player::White {
        result.sort_by(|a, b| score_game(b.new_game()).cmp(&score_game(a.new_game())));
    } else {
        result.sort_by(|a, b| score_game(a.new_game()).cmp(&score_game(b.new_game())));
    }
    result.into_iter().collect()
}

fn score_game(game: Game) -> i32 {
    game.list_pieces()
        .into_iter()
        .map(|piece| score_piece(piece))
        .sum()
}

fn score_piece(piece: Piece) -> i32 {
    let abs_score = match piece.piece_type() {
        PieceType::King => 100,
        PieceType::Queen => 900,
        PieceType::Rook => 500,
        PieceType::Bishop => 300,
        PieceType::Knight => 300,
        PieceType::Pawn => 100,
    };
    let direction = match piece.player() {
        Player::White => 1,
        Player::Black => -1,
    };
    abs_score * direction
}

#[cfg(test)]
mod tests {
    use engine::*;
    use game::Game;

    #[test]
    fn simple_run() {
        select_move(&Game::new());
    }
}
