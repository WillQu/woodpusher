use board::*;
use game::*;

pub fn generate_moves<'a>(game: &'a Game, player: Player, start_position: Position, directions: &[(i8, i8)]) -> Vector<Move<'a>> {
	let mut results = Vector::new();
	for (x, y) in directions {
		let mut position = start_position;
		loop {
			let new_position = Position::from_u8((position.column() as i8 + x) as u8, (position.row() as i8 + y) as u8);
			if new_position.is_none() {
				break;
			}
			let piece = game.get_piece_at(new_position.unwrap());
			if  piece.is_some() {
				if piece.unwrap().player() == player.opponent() {
					results.push_back(new_position.unwrap())
				}
				break;
			}
			results.push_back(new_position.unwrap());
			position = new_position.unwrap();
		}
	}
	results
		.iter()
		.map(|p| game.create_move(start_position, *p))
		.collect()
}