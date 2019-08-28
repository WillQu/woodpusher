use board::*;
use game::*;

pub fn list_rook_moves<'a>(game: &'a Game, position: Position, player: Player) -> Vector<Move<'a>> {
	move_list::generate_moves(game, player, position, &[(0, 1), (1, 0), (0, -1), (-1, 0)])
}

#[cfg(test)]
mod tests {
	use self::rook::*;
	
	use spectral::*;
	use im::HashSet;
	
	use board::*;
	use board::Player::*;
	use game::*;

	#[test]
	fn from_a1() {
		// Given
		let game = Game::from_board(Board::empty().put(Position::from("a1").unwrap(), Piece::new(PieceType::Rook, Player::White)), White);
		
		// When
		let result = list_rook_moves(&game, Position::from("a1").unwrap(), White);
		
		//Then
		let expected:HashSet<Position> = vector!["a2", "a3", "a4", "a5", "a6", "a7", "a8", "b1", "c1", "d1", "e1", "f1", "g1", "h1"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}
	
	#[test]
	fn from_h8() {
		// Given
		let game = Game::from_board(Board::empty().put(Position::from("h8").unwrap(), Piece::new(PieceType::Rook, Player::White)), White);
		
		// When
		let result = list_rook_moves(&game, Position::from("h8").unwrap(), White);
		
		//Then
		let expected:HashSet<Position> = vector!["h7", "h6", "h5", "h4", "h3", "h2", "h1", "a8", "b8", "c8", "d8", "e8", "f8", "g8"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}
	
	#[test]
	fn friend_in_the_way() {
		// Given
		let board = Board::empty()
			.put(Position::from("a1").unwrap(), Piece::new(PieceType::Rook, Player::White))
			.put(Position::from("a2").unwrap(), Piece::new(PieceType::Pawn, White));
		let game = Game::from_board(board, White);
		
		// When
		let result = list_rook_moves(&game, Position::from("a1").unwrap(), White);
		
		//Then
		let expected:HashSet<Position> = vector!["b1", "c1", "d1", "e1", "f1", "g1", "h1"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}
	
	#[test]
	fn friend_in_the_way2() {
		// Given
		let board = Board::empty()
			.put(Position::from("a1").unwrap(), Piece::new(PieceType::Rook, Player::White))
			.put(Position::from("b1").unwrap(), Piece::new(PieceType::Pawn, White));
		let game = Game::from_board(board, White);
		
		// When
		let result = list_rook_moves(&game, Position::from("a1").unwrap(), White);
		
		//Then
		let expected:HashSet<Position> = vector!["a2", "a3", "a4", "a5", "a6", "a7", "a8"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}
	
	#[test]
	fn enemy_in_the_way() {
		// Given
		let board = Board::empty()
			.put(Position::from("a1").unwrap(), Piece::new(PieceType::Rook, Player::White))
			.put(Position::from("a2").unwrap(), Piece::new(PieceType::Pawn, Black));
		let game = Game::from_board(board, White);
		
		// When
		let result = list_rook_moves(&game, Position::from("a1").unwrap(), White);
		
		//Then
		let expected:HashSet<Position> = vector!["b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}
	
	#[test]
	fn enemy_in_the_way2() {
		// Given
		let board = Board::empty()
			.put(Position::from("a1").unwrap(), Piece::new(PieceType::Rook, Player::White))
			.put(Position::from("b1").unwrap(), Piece::new(PieceType::Pawn, Black));
		let game = Game::from_board(board, White);
		
		// When
		let result = list_rook_moves(&game, Position::from("a1").unwrap(), White);
		
		//Then
		let expected:HashSet<Position> = vector!["a2", "a3", "a4", "a5", "a6", "a7", "a8", "b1"]
			.iter()
			.map(|pos| Position::from(pos).unwrap())
			.collect();
		let result_positions: HashSet<Position> = result.iter().map(|mv| mv.to).collect();
		assert_that!(result_positions).is_equal_to(expected);
	}
}