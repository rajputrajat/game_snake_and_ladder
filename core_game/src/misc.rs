#[derive(PartialEq, Debug)]
pub(crate) struct Position {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

#[derive(PartialEq, Debug)]
pub(crate) struct Movement {
    pub(crate) from: Position,
    pub(crate) to: Position,
}
