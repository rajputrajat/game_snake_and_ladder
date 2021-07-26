use crate::entity::Entity;
use crate::misc::Position;

pub(crate) struct Cell {
    entity: Option<Entity>,
    position: Position,
}
