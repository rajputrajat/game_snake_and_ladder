/// Game
use crate::board::Board;
use crate::player::{Player, PlayerState};

pub struct Game {
    board: Board,
    players: Vec<Player>,
}
