use game::Game;
use game::Move;

pub fn select_move<'a>(game: &'a Game) -> Option<Move<'a>> {
    let moves = game.list_moves();
    let result = moves.head();
    result.map(|m| *m)
}
