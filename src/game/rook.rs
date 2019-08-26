use board::*;
use game::*;

pub fn list_rook_moves<'a>(game: &'a Game, position: Position, player: Player) -> Vector<Move<'a>> {
	let columns = generate_line(game, player, position.row(), |x| Position::from_u8(position.column(), x));
	let rows = generate_line(game, player, position.column(), |x| Position::from_u8(x, position.row()));
	
	(rows + columns)
		.into_iter()
		.filter(|p| *p != position)
		.map(|pos| game.create_move(position, pos))
		.collect()
}

fn generate_line(game: &Game, player: Player, index: u8, position_gen: impl Fn(u8) -> Option<Position>) -> Vector<Position> {
	let mut results = Vector::new();
	for increment in &[-1, 1] {
		let mut index = index;
		loop {
			index = (index as i8 + increment) as u8;
			let new_position = position_gen(index);
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
		}
	}
	results
}

#[cfg(test)]
mod tests {
	use self::rook::*;
	
	use spectral::*;
	use spectral::prelude::ContainingIntoIterAssertions;
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