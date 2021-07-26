//! Game board
use crate::misc::{Movement, Position};
use anyhow::{anyhow, Result};
use log::{error, info, trace};
use rand::prelude::*;

pub(crate) struct Board {
    sq_of_side: u8,
    entities: Vec<Entity>,
}

#[derive(PartialEq, Debug)]
pub enum Entity {
    Snake(Movement),
    Ladder(Movement),
}

impl Entity {
    pub fn get_randomly(side_length: u8, count: u8) -> Vec<Entity> {
        const START: Position = Position { x: 0, y: 0 };
        let stop = Position {
            x: 0,
            y: (side_length - 1),
        };
        println!("testing");
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
                    let y_to = rng.gen_range((y_from + 1)..side_length);
                    Entity::Ladder(Movement {
                        from: Position {
                            x: x_from,
                            y: y_from,
                        },
                        to: Position { x: x_to, y: y_to },
                    })
                };
                if !entities.iter().any(|x| x.is_related(&entity)) {
                    match &entity {
                        Entity::Snake(m) | Entity::Ladder(m) => {
                            if m.from.ne(&START)
                                && m.to.ne(&START)
                                && m.from.ne(&stop)
                                && m.to.ne(&stop)
                            {
                                entities.push(entity);
                                return;
                            }
                        }
                    }
                }
            }
        });
        info!("{:?}", entities);
        entities
    }

    fn is_related(&self, ent: &Entity) -> bool {
        let in_m = match ent {
            Entity::Snake(m) | Entity::Ladder(m) => m,
        };
        match self {
            Entity::Snake(self_m) | Entity::Ladder(self_m) => {
                (self_m.from == in_m.from)
                    || (self_m.from == in_m.to)
                    || (self_m.to == in_m.from)
                    || (self_m.to == in_m.to)
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn get_entities() {
        init_logger();
        let v: Vec<Entity> = Vec::new();
        assert_eq!(v, Entity::get_randomly(10, 5));
    }
}
