/// Game board
use crate::misc::{Movement, Position};
use anyhow::{anyhow, Result};
use rand::prelude::*;

pub(crate) struct Board {
    sq_of_side: u8,
    entities: Vec<Entity>,
}

#[derive(PartialEq, Debug)]
enum Entity {
    Snake(Movement),
    Ladder(Movement),
}

impl Entity {
    fn get_randomly(side_length: u8, count: u8) -> Vec<Entity> {
        assert!(count < side_length.pow(2));
        let mut entities: Vec<Entity> = Vec::new();
        let mut rng = thread_rng();
        (0..count).into_iter().for_each(|_| loop {
            loop {
                let x_from = rng.gen_range(0..side_length);
                let x_to = rng.gen_range(0..side_length);
                let entity = if rand::random() {
                    let y_from = rng.gen_range(1..side_length);
                    let y_to = rng.gen_range(0..y_from);
                    Entity::Snake(Movement {
                        from: Position {
                            x: x_from,
                            y: y_from,
                        },
                        to: Position { x: x_to, y: y_to },
                    })
                } else {
                    let y_from = rng.gen_range(0..(side_length - 1));
                    let y_to = rng.gen_range(1..y_from);
                    Entity::Ladder(Movement {
                        from: Position {
                            x: x_from,
                            y: y_from,
                        },
                        to: Position { x: x_to, y: y_to },
                    })
                };
                if !entities.iter().any(|x| x == &entity) {
                    entities.push(entity);
                    break;
                }
            }
        });
        entities
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
