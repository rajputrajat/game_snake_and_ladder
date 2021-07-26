//! # Core game logic goes in here
//!
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod abilities;
mod board;
mod cell;
mod entity;
mod game;
mod misc;
mod player;

pub use crate::player::{Player, PlayerAction, PlayerId};
use anyhow::Result;

pub struct GameCore {
    pub players: Vec<PlayerId>,
}

impl GameCore {
    pub fn update() {}

    pub fn add_player(player: Player) -> PlayerId {}

    pub fn remove_player(player_id: PlayerId) -> Result<()> {}

    pub fn create_board(side_length: SideLength) -> Result<()> {}

    pub fn action(player_id: PlayerId, player_action: PlayerAction) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
