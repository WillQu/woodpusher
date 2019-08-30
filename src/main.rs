extern crate im;
extern crate woodpusher;

use std::io;

use im::Vector;

use woodpusher::board::Position;
use woodpusher::game::Game;
use woodpusher::game_cli;

fn main() -> io::Result<()> {
    let mut game = Game::new();
    while !game.list_moves().is_empty() {
        println!("{}", game_cli::show_board(game.board()));
        let (from, to) = ask_position()?;
        let game_result = game.execute_move(from, to);
        game = match game_result {
            Ok(g) => g,
            Err(_) => game,
        }
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
