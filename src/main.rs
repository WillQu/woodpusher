extern crate im;
extern crate woodpusher;

use std::io;

use im::Vector;

use woodpusher::board::Position;
use woodpusher::engine;
use woodpusher::game::Game;
use woodpusher::game_cli;

fn main() -> io::Result<()> {
    let mut game = Game::new();
    while !game.list_moves().is_empty() {
        println!("{}", game_cli::show_board(game.board()));
        game = {
            let moves = engine::select_move(&game);
            moves.head().map_or(game.clone(), |mv| mv.new_game())
        };
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
    }
    Ok(())
}

fn ask_position() -> io::Result<(Position, Position)> {
    let mut result = None;
    while result.is_none() {
        println!("Enter start and destination positions");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input_list: Vector<&str> = input.trim().split(' ').collect();
        if input_list.len() == 2 {
            let tuple_result = (Position::from(input_list[0]), Position::from(input_list[1]));
            result = match tuple_result {
                (Some(_), Some(_)) => Some(tuple_result),
                _ => None,
            }
        }
    }
    let (from, to) = result.unwrap();
    Ok((from.unwrap(), to.unwrap()))
}
