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
    pub fn create_board(&mut self, side_length: SideLength) -> Result<()> {
        self.board = Board::new(side_length);
        Ok(())
    }

    /// add a player on board
    pub fn add_player(&mut self, joining_player: Player) -> PlayerId {
        let joining_player_id = self.board.join(joining_player);
        self.players.push(joining_player_id);
        joining_player_id
    }

    /// remove player at any time
    pub fn remove_player(&mut self, leaving_player_id: PlayerId) -> Result<()> {
        self.board.remove(leaving_player_id)
    }

    /// get player info back
    pub fn get_player(&self, check_player_id: PlayerId) -> Result<Player> {
        self.board.read_player_info(check_player_id)
    }

    /// player action
    pub fn action(&mut self, player_id: PlayerId, player_action: PlayerAction) -> Result<()> {
        self.board.action(player_id, player_action)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
