use crate::entity::Entity;
use crate::misc::Position;

#[derive(PartialEq, Debug)]
pub(crate) struct CellId(pub(crate) u8);

pub(crate) struct Cell {
    entity: Option<Entity>,
    position: Position,
}
