//! # Core game logic goes in here
//!
#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod board;
mod game;
mod misc;
mod player;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
