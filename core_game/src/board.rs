//! Game board
use crate::{
    cell::{Cell, CellId},
    entity::Entity,
    player::{Player, PlayerId},
};
use anyhow::{anyhow, Result};
use log::{error, info, trace};
use std::collections::HashMap;

pub struct SideLength(pub(crate) u8);
impl SideLength {
    #[inline]
    pub(crate) fn sq(self) -> u8 {
        self.0.pow(2)
    }
}

pub(crate) struct Board {
    side_length: SideLength,
    players: HashMap<PlayerId, (Option<Player>, Option<CellId>)>,
    cells: Vec<Cell>,
    current_player: Option<PlayerId>,
}

// for associated functions
impl Board {
    pub(crate) fn new(side_length: SideLength) -> Self {
        Self {
            side_length,
            cells: Vec::new(),
            current_player: None,
            players: HashMap::new(),
        }
    }
}

// public functions
impl Board {
    pub(crate) fn join(&mut self, player: Player) -> PlayerId {
        trace!("joining player: {:?}", player);
        let players_count = self.players.len();
        self.players
            .insert(PlayerId(players_count as u8), (Some(player), None));
        PlayerId(self.players.len() as u8 - 1)
    }

    pub(crate) fn remove(&mut self, player_id: PlayerId) {
        let mut leaving_player = &self.players[&player_id];
        trace!("player leaving: {:?}", leaving_player);
        leaving_player = &(None, None);
    }

    pub(crate) fn action(&self, player_id: PlayerId) -> Result<()> {
        Ok(())
    }
}

// private functions
impl Board {
    fn check_piece_overlap(&self, player_id: PlayerId) -> Option<PlayerId> {
        if let (_, Some(p_cell)) = self.players[&player_id] {
            let overlapped_players: Vec<PlayerId> = self
                .players
                .iter()
                .filter_map(|(&p_id, &(_, c))| {
                    if let Some(c) = c {
                        if p_cell == c {
                            return Some(p_id);
                        }
                    }
                    None
                })
                .collect();
            assert!(overlapped_players.len() <= 1);
            if overlapped_players.len() == 1 {
                return Some(overlapped_players[0]);
            }
        }
        None
    }

    fn play_turn(player_id: PlayerId) {}
    fn create_cells() {}
    fn create_entities() {}
    fn push_entities() {}
}

enum MoveType {
    Normal,
    Jump(Entity),
}
