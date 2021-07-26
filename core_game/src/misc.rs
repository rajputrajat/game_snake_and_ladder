#[derive(PartialEq, Debug)]
pub struct Position {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

#[derive(PartialEq, Debug)]
pub struct Movement {
    pub(crate) from: Position,
    pub(crate) to: Position,
}
