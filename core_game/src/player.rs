/// Player
use crate::abilities::Ability;
use anyhow::{anyhow, Result};

/// player-id. Index for `Player` vector
#[derive(PartialEq, Hash, Debug, Eq, Copy, Clone)]
pub struct PlayerId(pub u8);

/// player data
#[derive(Debug, Clone)]
pub struct Player {
    state: PlayerState,
    name: String,
    color: String,
    abilities: Vec<Ability>,
}

impl Player {
    pub(crate) fn add_ability(&mut self, ability: Ability) {
        self.abilities.push(ability)
    }

    pub(crate) fn has_ability(&self, ability: Ability) -> bool {
        self.abilities.iter().any(|a| a == &ability)
    }

    pub(crate) fn remove_ability(&mut self, ability: Ability) -> Result<()> {
        let ability_index = self
            .abilities
            .iter()
            .enumerate()
            .find_map(|(i, a)| if a == &ability { Some(i) } else { None })
            .ok_or_else(|| anyhow!("ability '{:?}' wasn't fount!", &ability))?;
        self.abilities.remove(ability_index);
        Ok(())
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Dice {
    Normal,
    SuperDice8,
    SuperDice10,
    SuperDice12,
}

impl Dice {
    pub fn num_faces(&self) -> u8 {
        match &self {
            Dice::Normal => 6,
            Dice::SuperDice8 => 8,
            Dice::SuperDice10 => 10,
            Dice::SuperDice12 => 12,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum PlayerState {
    Idle,
    InPlay,
    Won,
}

/// all possible player actions
#[derive(Debug, Clone, Copy)]
pub enum PlayerAction {
    /// Roll the dice
    RollDice(Dice),
    /// play ability
    UseAbility(Ability),
}
