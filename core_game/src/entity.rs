use crate::{
    abilities::Ability,
    board::SideLength,
    cell::CellId,
    misc::{Movement, Position},
};
use rand::{prelude::*, rngs::ThreadRng};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub(crate) enum Entity {
    Snake(Movement),
    Ladder(Movement),
    Ability(Ability),
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

    pub(crate) fn create_assorted(mut self, side_length: u8) -> HashMap<CellId, Entity> {
        let mut rng = thread_rng();
        self.entities
    }

    fn find_empty_random_lower(&self, rng: &mut ThreadRng, cell_id: CellId) -> CellId {}
    fn find_empty_random_upper(&self, rng: &mut ThreadRng, cell_id: CellId) -> CellId {}
    fn find_empty_random(&self, rng: &mut ThreadRng) -> CellId {
        loop {
            let cell = rng.gen_range(1..(self.side_length.eq() - 1));
            if !self.entities.keys().iter().any(|x| x == cell) {
                return cell;
            }
        }
    }
}
