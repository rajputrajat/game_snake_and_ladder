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

pub use crate::{
    board::SideLength,
    player::{Player, PlayerAction, PlayerId},
};
use anyhow::Result;

pub struct GameCore {
    pub players: Vec<PlayerId>,
}

impl GameCore {
    pub fn update() {}

    pub fn add_player(player: Player) -> PlayerId {
        PlayerId(0)
    }

    pub fn remove_player(player_id: PlayerId) -> Result<()> {
        Ok(())
    }

    pub fn create_board(side_length: SideLength) -> Result<()> {
        Ok(())
    }

    pub fn action(player_id: PlayerId, player_action: PlayerAction) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
