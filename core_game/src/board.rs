use crate::misc::{Movement, Position};
use anyhow::{anyhow, Result};
/// Game board
use rand::prelude::*;

pub(crate) struct Board {
    sq_of_side: u8,
    entities: Vec<Entity>,
}

enum Entity {
    Snake(Option<Movement>),
    Ladder(Option<Movement>),
}

impl Entity {
    fn does_pos_overlap(&self, pos: Position) -> Result<bool> {
        match &self {
            Entity::Snake(Some(m)) | Entity::Ladder(Some(m)) => Ok(pos == m.from || pos == m.to),
            _ => Err(anyhow!("this entity isn't initialized")),
        }
    }
}

impl Entity {
    fn get_randomly(side_length: u8, count: u8) -> Vec<Entity> {
        assert!(count < side_length.pow(2));
        let mut entities: Vec<Entity> = Vec::new();
        (0..count).into_iter().for_each(|_| loop {
            kkkk
            if entities.iter().find(|x| )
        });
    }
}

enum MoveType {
    Normal,
    Jump(Entity),
}

impl Board {
    pub(crate) fn new(side_length: u8) -> Self {
        Self {
            sq_of_side: side_length,
            entities: Board::spawn_entities(side_length),
        }
    }

    pub(crate) fn r#move(cur_pos: Position) -> Position {
        Position { x: 0, y: 0 }
    }
}

impl Board {
    fn spawn_entities(side_length: u8) -> Vec<Entity> {
        vec![]
    }
}
