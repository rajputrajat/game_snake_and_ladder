use crate::entity::Entity;
use crate::misc::Position;

#[derive(PartialEq, Eq, PartialOrd, Hash, Debug)]
pub(crate) struct CellId(pub(crate) u8);

pub(crate) struct Cell {
    entity: Option<Entity>,
    position: Position,
}
