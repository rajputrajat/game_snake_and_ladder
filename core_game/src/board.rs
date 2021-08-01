//! Game board
use crate::{
    abilities::Ability,
    cell::{Cell, CellId},
    entity::Entity,
    player::{Dice, Player, PlayerAction, PlayerId},
};
use anyhow::{anyhow, Result};
use log::trace;
use rand::prelude::*;
use std::collections::HashMap;

/// Board is a square of value of this type
pub struct SideLength(pub(crate) u8);

pub(crate) struct Board {
    side_length: SideLength,
    players: HashMap<PlayerId, (Option<Player>, Option<CellId>)>,
    cells: Vec<Cell>,
    current_player: Option<PlayerId>,
    rng: ThreadRng,
    state: StateMachine,
}

#[derive(Debug, PartialEq)]
enum StateMachine {
    Init,
    HandlePlayerAction,
    Move,
    CheckEntity,
    CheckOverlap,
    Win,
}

impl StateMachine {
    fn goto(&mut self, next_state: StateMachine) {
        *self = next_state;
    }
}

// for associated functions
impl Board {
    pub(crate) fn new(side_length: SideLength) -> Self {
        Self {
            side_length,
            cells: Vec::new(),
            current_player: None,
            players: HashMap::new(),
            rng: rand::thread_rng(),
            state: StateMachine::Init,
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

    pub(crate) fn remove(&mut self, player_id: PlayerId) -> Result<()> {
        if let Some(v) = self.players.get_mut(&player_id) {
            trace!("player leaving: {:?}", v);
            *v = (None, None);
            Ok(())
        } else {
            Err(anyhow!("this player -> '{:?}' doesn't exist!", &player_id))
        }
    }

    pub(crate) fn action(&mut self, player_id: PlayerId, action: PlayerAction) -> Result<()> {
        assert_eq!(self.state, StateMachine::Init);
        loop {
            match &self.state {
                StateMachine::HandlePlayerAction => match &action {
                    PlayerAction::RollDice(dice) => {
                        let outcome = self.roll_dice(dice);
                    }
                    PlayerAction::UseAbility(ability) => {
                        match &ability {
                            Ability::SuperDice(dice) => {}
                            Ability::CustomSnLdMaker(Some(movement)) => {}
                            _ => {}
                        }
                        self.get_player_mut(player_id)?.remove_ability(*ability)?;
                    }
                },
                StateMachine::Move => {
                    break;
                }
                StateMachine::Init => {}
                StateMachine::CheckEntity => {}
                StateMachine::CheckOverlap => {}
                StateMachine::Win => {}
            }
        }
        Ok(())
    }

    pub(crate) fn read_player_info(&self, check_player: PlayerId) -> Result<Player> {
        self.players[&check_player]
            .0
            .clone()
            .ok_or_else(|| anyhow!("Player '{:?}' doesn't exist!", &check_player))
    }
}

// private functions
impl Board {
    fn get_player(&self, player_id: PlayerId) -> Result<&Player> {
        self.players
            .get(&player_id)
            .ok_or_else(|| anyhow!("invalid player '{:?}'", player_id))?
            .0
            .as_ref()
            .ok_or_else(|| anyhow!("player: '{:?}' has left the game.", player_id))
    }

    fn get_player_mut(&mut self, player_id: PlayerId) -> Result<&mut Player> {
        self.players
            .get_mut(&player_id)
            .ok_or_else(|| anyhow!("invalid player '{:?}'", player_id))?
            .0
            .as_mut()
            .ok_or_else(|| anyhow!("player: '{:?}' has left the game.", player_id))
    }

    /// check if this player's piece is overlapping some other's
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

    /// play turn for player
    fn play_turn(&mut self, player_id: PlayerId) {}

    fn roll_dice(&mut self, dice: &Dice) -> u8 {
        self.rng.gen_range(1..dice.num_faces())
    }

    fn create_cells() {}
    fn create_entities() {}
    fn push_entities() {}
}

enum MoveType {
    Normal,
    Jump(Entity),
}
