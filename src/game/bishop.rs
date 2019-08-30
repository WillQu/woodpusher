use board::*;
use game::*;

pub fn list_bishop_moves(game: &Game, position: Position, player: Player) -> Vector<Move<'_>> {
	move_list::generate_moves(game, player, position, &[(1, 1), (-1, 1), (1, -1), (-1, -1)])
}

#[cfg(test)]
mod tests {
	use self::bishop::*;
	
	use spectral::*;
	use im::HashSet;
	
	use board::*;
	use board::Player::*;
	use game::*;

	#[test]
	fn from_a1() {
		// Given
		let game = Game::from_board(Board::empty().put(Position::from("a1").unwrap(), Piece::new(PieceType::Bishop, Player::White)), White);
		
		// When
		let result = list_bishop_moves(&game, Position::from("a1").unwrap(), White);

		//Then
		let expected:HashSet<Position> = vector!["b2", "c3", "d4", "e5", "f6", "g7", "h8"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}

	#[test]
	fn from_a8() {
		// Given
		let game = Game::from_board(Board::empty().put(Position::from("a8").unwrap(), Piece::new(PieceType::Bishop, Player::White)), White);
		
		// When
		let result = list_bishop_moves(&game, Position::from("a8").unwrap(), White);

		//Then
		let expected:HashSet<Position> = vector!["b7", "c6", "d5", "e4", "f3", "g2", "h1"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}
}