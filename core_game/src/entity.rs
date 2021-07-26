use crate::abilities::Ability;
use crate::misc::{Movement, Position};

#[derive(PartialEq, Debug)]
pub(crate) enum Entity {
    Snake(Movement),
    Ladder(Movement),
    Ability(Ability),
}
