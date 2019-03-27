use board::Board;
use board::Position;
use board::Piece;
use board::PieceType::*;
use board::Player::*;

pub fn show_board(board: &Board) -> String {
    let mut result = String::new();
    for line in "87654321".chars() {
        for row in "abcdefgh".chars() {
            result.push(board
                .get(&Position::from(&[row, line].into_iter().collect::<String>()).unwrap())
                .map_or_else(|| '·', |piece| show_piece(piece))
            );
            result.push(' ');
        }
        result.pop();
        result.push('\n');
    }
    result
}

fn show_piece(piece: &Piece) -> char {
    match (piece.piece_type(), piece.player()) {
        (Rook, Black) => '♜',
        (Knight, Black) => '♞',
        (Bishop, Black) => '♝',
        (Queen, Black) => '♛',
        (King, Black) => '♚',
        (Pawn, Black) => '♟',
        (Rook, White) => '♖',
        (Knight, White) => '♘',
        (Bishop, White) => '♗',
        (Queen, White) => '♕',
        (King, White) => '♔',
        (Pawn, White) => '♙',
    }
}

#[cfg(test)]
mod tests {
    use game_cli::*;

    #[test]
    fn print_new_board() {
        // Given
        let board = Board::starting_position();

        // When
        let result = show_board(&board);

        // Then
        assert_eq!(result, "♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
· · · · · · · ·
· · · · · · · ·
· · · · · · · ·
· · · · · · · ·
♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
")
    }
}
