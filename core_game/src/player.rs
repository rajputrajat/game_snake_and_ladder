/// Player
use crate::abilities::Ability;
use crate::entity::Movement;

#[derive(PartialEq, Hash, Debug, Eq, Copy, Clone)]
pub struct PlayerId(pub u8);

#[derive(Debug)]
pub struct Player {
    state: PlayerState,
    name: String,
    color: String,
    abilities: Vec<Ability>,
}

pub struct SuperDiceFaces(pub(crate) u8);

#[derive(Debug)]
pub(crate) enum PlayerState {
    Idle,
    InPlay,
    Won,
}

pub enum PlayerAction {
    RollDice,
    RollSuperDice(SuperDiceFaces),
    MakeCustomEntity(Movement),
}
