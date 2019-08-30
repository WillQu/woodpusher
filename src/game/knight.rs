use board::*;
use game::*;

pub fn list_knight_moves(game: &Game, position: Position, player: Player) -> Vector<Move<'_>> {
	move_list::generate_moves_one_square(game, player, position, &[(1, 2), (2, 1), (1, -2), (2, -1), (-1, -2), (-2, -1), (-1, 2), (-2, 1)])
}

#[cfg(test)]
mod tests {
	use self::knight::*;
	
	use spectral::*;
	use im::HashSet;

	use board::*;
	use board::Player::*;
	use game::*;

	#[test]
	fn from_a1() {
		// Given
		let game = Game::from_board(Board::empty().put(Position::from("a1").unwrap(), Piece::new(PieceType::Knight, Player::White)), White);
		
		// When
		let result = list_knight_moves(&game, Position::from("a1").unwrap(), White);
		
		//Then
		let expected: HashSet<Position> = ["b3", "c2"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}

	#[test]
	fn from_a8() {
		// Given
		let game = Game::from_board(Board::empty().put(Position::from("a8").unwrap(), Piece::new(PieceType::Knight, Player::White)), White);
		
		// When
		let result = list_knight_moves(&game, Position::from("a8").unwrap(), White);
		
		//Then
		let expected: HashSet<Position> = ["b6", "c7"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}

	#[test]
	fn from_h8() {
		// Given
		let game = Game::from_board(Board::empty().put(Position::from("h8").unwrap(), Piece::new(PieceType::Knight, Player::White)), White);
		
		// When
		let result = list_knight_moves(&game, Position::from("h8").unwrap(), White);
		
		//Then
		let expected: HashSet<Position> = ["f7", "g6"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}

	#[test]
	fn from_h1() {
		// Given
		let game = Game::from_board(Board::empty().put(Position::from("h1").unwrap(), Piece::new(PieceType::Knight, Player::White)), White);
		
		// When
		let result = list_knight_moves(&game, Position::from("h1").unwrap(), White);
		
		//Then
		let expected: HashSet<Position> = ["f2", "g3"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}
}