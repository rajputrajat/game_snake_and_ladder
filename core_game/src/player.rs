/// Player
use crate::abilities::Ability;

/// player-id. Index for `Player` vector
#[derive(PartialEq, Hash, Debug, Eq, Copy, Clone)]
pub struct PlayerId(pub u8);

/// player data
#[derive(Debug)]
pub struct Player {
    state: PlayerState,
    name: String,
    color: String,
    abilities: Vec<Ability>,
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

#[derive(Debug)]
pub(crate) enum PlayerState {
    Idle,
    InPlay,
    Won,
}

/// all possible player actions
pub enum PlayerAction {
    /// Roll the dice
    RollDice(Dice),
    /// play ability
    UseAbility(Ability),
}
