use im::Vector;
use rayon::prelude::*;
use std::cmp::*;

use board::Piece;
use board::PieceType;
use board::Player;
use game::Game;
use game::Move;

pub fn select_move(game: &Game, depth: i32) -> (Option<Move>, i32) {
    alpha_beta(game, i32::MIN, i32::MAX, depth)
}

fn alpha_beta(game: &Game, alpha: i32, beta: i32, depth: i32) -> (Option<Move>, i32) {
    if depth <= 0 {
        (None, score_game(game))
    } else {
        let candidates = game.list_moves();
        if candidates.len() == 0 {
            (None, score_game(game))
        } else if game.turn() == Player::Black {
            let mut beta = beta;
            let mut m = None;
            for candidate in candidates {
                let (_, result) = alpha_beta(&candidate.new_game(), alpha, beta, depth - 1);
                if result <= alpha {
                    return (None, alpha);
                }
                if result < beta {
                    beta = result;
                    m = Some(candidate);
                }
            }
            (m, beta)
        } else {
            let mut alpha = alpha;
            let mut m = None;
            for candidate in candidates {
                let (_, result) = alpha_beta(&candidate.new_game(), alpha, beta, depth - 1);
                if result >= beta {
                    return (None, beta);
                }
                if result > alpha {
                    alpha = result;
                    m = Some(candidate);
                }
            }
            (m, alpha)
        }
    }
}

fn score_game(game: &Game) -> i32 {
    if game.is_stalemate() {
        0
    } else if game.is_mate() {
        match game.turn() {
            Player::White => -20000,
            Player::Black => 20000,
        }
    } else {
        let mut result = game
            .list_pieces()
            .into_iter()
            .map(|piece| score_piece(piece))
            .sum();
        result += score_moves(game);
        result
    }
}

fn score_moves(game: &Game) -> i32 {
    ((game.set_turn(Player::White)).list_moves().len() as i32
        - (game.set_turn(Player::Black)).list_moves().len() as i32)
        * 10
}

fn score_piece(piece: Piece) -> i32 {
    let abs_score = match piece.piece_type() {
        PieceType::King => 20000,
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
        select_move(&Game::new(), 1);
    }
}
