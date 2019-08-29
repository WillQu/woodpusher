use board::*;
use game::*;

pub fn list_queen_moves<'a>(game: &'a Game, position: Position, player: Player) -> Vector<Move<'a>> {
	move_list::generate_moves(game, player, position, &[(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, 1), (1, -1), (-1, -1)])
}

#[cfg(test)]
mod tests {
	use self::queen::*;
	
	use spectral::*;
	use im::HashSet;

	use board::*;
	use board::Player::*;
	use game::*;

	#[test]
	fn from_a1() {
		// Given
		let game = Game::from_board(Board::empty().put(Position::from("a1").unwrap(), Piece::new(PieceType::Queen, Player::White)), White);
		
		// When
		let result = list_queen_moves(&game, Position::from("a1").unwrap(), White);
		
		//Then
		let expected: HashSet<Position> = ["a2", "a3", "a4", "a5", "a6", "a7", "a8", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "b2", "c3", "d4", "e5", "f6", "g7", "h8"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}

	#[test]
	fn from_a8() {
		// Given
		let game = Game::from_board(Board::empty().put(Position::from("a8").unwrap(), Piece::new(PieceType::Queen, Player::White)), White);
		
		// When
		let result = list_queen_moves(&game, Position::from("a8").unwrap(), White);
		
		//Then
		let expected: HashSet<Position> = ["a1", "a2", "a3", "a4", "a5", "a6", "a7", "b8", "c8", "d8", "e8", "f8", "g8", "h8", "b7", "c6", "d5", "e4", "f3", "g2", "h1"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}
}