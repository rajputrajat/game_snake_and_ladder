use crate::{abilities::Ability, board::SideLength, cell::CellId};
use anyhow::{anyhow, Result};
use rand::{prelude::*, rngs::ThreadRng};
use std::{collections::HashMap, ops::Range};

#[derive(PartialEq, Debug)]
pub(crate) enum Entity {
    Snake(Movement),
    Ladder(Movement),
    Ability(Ability),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Movement {
    pub(crate) from: CellId,
    pub(crate) to: CellId,
}

pub(crate) struct EntityFactory {
    side_length: SideLength,
    entities: HashMap<CellId, Entity>,
}

impl EntityFactory {
    pub(crate) fn new(side_length: SideLength) -> Self {
        Self {
            entities: HashMap::new(),
            side_length,
        }
    }

    pub(crate) fn create_assorted(mut self, side_length: u8) -> Result<HashMap<CellId, Entity>> {
        let mut rng = thread_rng();

        (0..rng.gen_range(8..12)).try_for_each(|x| -> Result<()> {
            let lower = self.find_empty_random_upper(&mut rng, CellId(1))?;
            if rand::random() {
                self.entities.insert(
                    lower,
                    Entity::Snake(Movement {
                        from: lower,
                        to: self.find_empty_random_lower(&mut rng, lower)?,
                    }),
                );
            }
            Ok(())
        })?;
        Ok(self.entities)
    }

    fn find_empty_random_lower(&self, rng: &mut ThreadRng, cell_id: CellId) -> Result<CellId> {
        if cell_id.0 < self.side_length.0 {
            Err(anyhow!("cell_id: {:?} is lowest row", cell_id))
        } else {
            let lower_row = (cell_id.0 / self.side_length.0) - 1;
            Ok(self.find_empty_random(rng, 1..((lower_row + 1) * self.side_length.0 - 1)))
        }
    }

    fn find_empty_random_upper(&self, rng: &mut ThreadRng, cell_id: CellId) -> Result<CellId> {
        if cell_id.0 > (self.side_length.0 * (self.side_length.0 - 1) - 1) {
            Err(anyhow!("cell_id: {:?} is upper-most row", cell_id))
        } else {
            let upper_row = (cell_id.0 / self.side_length.0) + 1;
            Ok(self.find_empty_random(
                rng,
                (upper_row * self.side_length.0)..(self.side_length.0 * self.side_length.0 - 1),
            ))
        }
    }

    fn find_empty_random(&self, rng: &mut ThreadRng, range: Range<u8>) -> CellId {
        loop {
            let cell = CellId(rng.gen_range(range.clone()));
            if !self.entities.keys().any(|x| x == &cell) {
                return cell;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_entity_factory() -> EntityFactory {
        let mut e_factory = EntityFactory::new(SideLength(10_u8));
        let dummy_pos = Position { x: 0, y: 0 };
        let dummy_mov = Movement {
            from: dummy_pos,
            to: dummy_pos,
        };
        (0_u8..(10_u8 * 10_u8)).for_each(|x| {
            if x % 2 == 0 {
                let _ = e_factory
                    .entities
                    .insert(CellId(x), Entity::Snake(dummy_mov));
            }
        });
        e_factory
    }

    #[test]
    fn empty() {
        let e_factory = create_entity_factory();
        let mut rng = thread_rng();
        let found_cell_id = e_factory.find_empty_random(&mut rng, 1..99).0;
        assert_eq!(found_cell_id % 2, 1);
    }

    #[test]
    fn upper_1() {
        let e_factory = create_entity_factory();
        let mut rng = thread_rng();
        let found_cell_id = e_factory
            .find_empty_random_upper(&mut rng, CellId(29))
            .unwrap()
            .0;
        println!("{:?}", found_cell_id);
        assert_eq!(found_cell_id % 2, 1);
        assert!(found_cell_id > 29);
    }

    #[test]
    fn upper_2() {
        let e_factory = create_entity_factory();
        let mut rng = thread_rng();
        let found_cell_id = e_factory
            .find_empty_random_upper(&mut rng, CellId(88))
            .unwrap()
            .0;
        println!("{:?}", found_cell_id);
        assert_eq!(found_cell_id % 2, 1);
        assert!(found_cell_id > 89);
    }

    #[test]
    fn upper_3() {
        let e_factory = create_entity_factory();
        let mut rng = thread_rng();
        let found_cell_id = e_factory.find_empty_random_upper(&mut rng, CellId(90));
        assert!(found_cell_id.is_err());
    }

    #[test]
    fn lower_1() {
        let e_factory = create_entity_factory();
        let mut rng = thread_rng();
        let found_cell_id = e_factory
            .find_empty_random_lower(&mut rng, CellId(20))
            .unwrap()
            .0;
        assert_eq!(found_cell_id % 2, 1);
        assert!(found_cell_id < 20);
    }

    #[test]
    fn lower_2() {
        let e_factory = create_entity_factory();
        let mut rng = thread_rng();
        let found_cell_id = e_factory
            .find_empty_random_lower(&mut rng, CellId(10))
            .unwrap()
            .0;
        assert_eq!(found_cell_id % 2, 1);
        assert!(found_cell_id < 10);
    }

    #[test]
    fn lower_3() {
        let e_factory = create_entity_factory();
        let mut rng = thread_rng();
        let found_cell_id = e_factory
            .find_empty_random_lower(&mut rng, CellId(99))
            .unwrap()
            .0;
        assert_eq!(found_cell_id % 2, 1);
        assert!(found_cell_id < 90);
    }

    #[test]
    fn lower_4() {
        let e_factory = create_entity_factory();
        let mut rng = thread_rng();
        let found_cell_id = e_factory.find_empty_random_lower(&mut rng, CellId(9));
        assert!(found_cell_id.is_err());
    }
}
