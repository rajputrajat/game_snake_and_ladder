/// Game
use crate::board::{Board, Entity};
use crate::player::{Player, PlayerState};

pub struct Game {
    board: Board,
    players: Vec<Player>,
}
