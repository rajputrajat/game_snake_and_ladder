/// Player
use crate::abilities::Ability;

pub(crate) struct Player {
    state: PlayerState,
    name: String,
    color: String,
    abilities: Vec<Ability>,
}

pub(crate) enum PlayerState {
    Idle,
    InPlay,
    Won,
}
