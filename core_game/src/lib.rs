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

use crate::board::Board;
pub use crate::{
    board::SideLength,
    player::{Player, PlayerAction, PlayerId},
};
use anyhow::Result;

/// Game core functionality
pub struct GameCore {
    /// player ids
    pub players: Vec<PlayerId>,
    /// board
    board: Board,
}

impl GameCore {
    /// create game board
    pub fn create_board(side_length: SideLength) -> Result<()> {
        Ok(())
    }

    /// add a player on board
    pub fn add_player(player: Player) -> PlayerId {
        PlayerId(0)
    }

    /// remove player at any time
    pub fn remove_player(player_id: PlayerId) -> Result<()> {
        Ok(())
    }

    /// player action
    pub fn action(player_id: PlayerId, player_action: PlayerAction) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
