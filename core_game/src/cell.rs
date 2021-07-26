use crate::entity::Entity;

#[derive(PartialEq, Eq, PartialOrd, Hash, Debug, Copy, Clone)]
pub(crate) struct CellId(pub(crate) u8);

pub(crate) struct Cell {
    entity: Option<Entity>,
}
