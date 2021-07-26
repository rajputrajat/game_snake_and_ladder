#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Movement {
    pub(crate) from: Position,
    pub(crate) to: Position,
}
