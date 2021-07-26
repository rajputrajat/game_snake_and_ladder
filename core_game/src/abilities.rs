use crate::{entity::Movement, player::Dice};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Ability {
    SuperDice(Dice),
    CustomSnLdMaker(Option<Movement>),
}

pub(crate) const ABILITIES: [Ability; 4] = [
    Ability::CustomSnLdMaker(None),
    Ability::SuperDice(Dice::SuperDice8),
    Ability::SuperDice(Dice::SuperDice10),
    Ability::SuperDice(Dice::SuperDice12),
];
