use crate::entity::Entity;
use crate::misc::Position;

pub(crate) struct CellId(u8);

pub(crate) struct Cell {
    entity: Option<Entity>,
    position: Position,
}
