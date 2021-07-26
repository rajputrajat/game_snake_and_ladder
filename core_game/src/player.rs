/// Player
use crate::abilities::Ability;
use crate::entity::Movement;

pub(crate) struct PlayerId(u8);

pub(crate) struct Player {
    state: PlayerState,
    name: String,
    color: String,
    abilities: Vec<Ability>,
}

pub(crate) struct SuperDice(pub(crate) u8);

pub(crate) enum PlayerState {
    Idle,
    InPlay,
    Won,
}

pub(crate) enum PlayerAction {
    RollDice,
    RollSuperDice(SuperDice),
    MakeCustomEntity(Movement),
}
