use crate::{
    abilities::Ability,
    board::SideLength,
    cell::CellId,
    misc::{Movement, Position},
};
use anyhow::{anyhow, Result};
use rand::{prelude::*, rngs::ThreadRng};
use std::{collections::HashMap, ops::Range};

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

    fn find_empty_random_lower(&self, rng: &mut ThreadRng, cell_id: CellId) -> Result<CellId> {
        if cell_id.0 < self.side_length.0 {
            Err(anyhow!("cell_id: {:?} is lowest row", cell_id))
        } else {
            let lower_cell = (cell_id.0 / self.side_length.0) - 1;
            Ok(self.find_empty_random(rng, 1..(lower_cell * self.side_length.0)))
        }
    }

    fn find_empty_random_upper(&self, rng: &mut ThreadRng, cell_id: CellId) -> Result<CellId> {
        if cell_id.0 > (self.side_length.0 * (self.side_length.0 - 1)) {
            Err(anyhow!("cell_id: {:?} is upper-most row", cell_id))
        } else {
            let upper_cell = (cell_id.0 / self.side_length.0) + 1;
            Ok(self.find_empty_random(rng, upper_cell..self.side_length.0))
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
