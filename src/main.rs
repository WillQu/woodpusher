extern crate woodpusher;

use woodpusher::board;
use woodpusher::game_cli;

fn main() {
    println!("{}", game_cli::show_board(&board::Board::starting_position()));
}