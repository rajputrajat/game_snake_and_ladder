/// Player
pub(crate) struct Player {
    state: PlayerState,
    cur_pos: u16,
}

pub(crate) enum PlayerState {
    Idle,
    InPlay,
    Won,
}
