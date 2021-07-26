//! # Core game logic goes in here
//!
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod abilities;
pub mod board;
mod cell;
mod entity;
mod game;
mod misc;
mod player;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
