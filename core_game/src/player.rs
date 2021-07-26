/// Player
use crate::abilities::Ability;
use crate::entity::Movement;

pub struct PlayerId(pub u8);

pub struct Player {
    state: PlayerState,
    name: String,
    color: String,
    abilities: Vec<Ability>,
}

pub struct SuperDiceFaces(pub(crate) u8);

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
