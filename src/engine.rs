use im::Vector;

use board::Piece;
use board::PieceType;
use board::Player;
use game::Game;
use game::Move;

pub fn select_move<'a>(game: &'a Game) -> Vector<Move<'a>> {
    let mut result: Vec<Move<'a>> = game.list_moves().into_iter().collect();
    let depth = 2;
    if game.turn() == Player::White {
        result.sort_by(|a, b| minimax(&b.new_game(), depth).cmp(&minimax(&a.new_game(), depth)));
    } else {
        result.sort_by(|a, b| minimax(&a.new_game(), depth).cmp(&minimax(&b.new_game(), depth)));
    }
    result.into_iter().collect()
}

fn minimax(game: &Game, depth: i32) -> i32 {
    if depth <= 0 {
        score_game(game)
    } else {
        let candidates = game.list_moves();
        if candidates.len() == 0 {
            score_game(game)
        } else if game.turn() == Player::White {
            candidates
                .iter()
                .map(|mv| minimax(&mv.new_game(), depth - 1))
                .fold(i32::MIN, i32::max)
        } else {
            candidates
                .iter()
                .map(|mv| minimax(&mv.new_game(), depth - 1))
                .fold(i32::MAX, i32::min)
        }
    }
}

fn score_game(game: &Game) -> i32 {
    if game.is_stalemate() {
        0
    } else if game.is_mate() {
        match game.turn() {
            Player::White => -1000000,
            Player::Black => 1000000,
        }
    } else {
        game.list_pieces()
            .into_iter()
            .map(|piece| score_piece(piece))
            .sum()
    }
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

enum Tree {
    Leaf { game: Game, score: i32 },
    Node { children: Vector<Tree> },
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
