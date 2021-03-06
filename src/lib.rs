#[macro_use]
extern crate im;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate rayon;

#[cfg(test)]
extern crate spectral;

pub mod board;
pub mod engine;
pub mod game;
pub mod game_cli;
