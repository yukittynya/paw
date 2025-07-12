use crate::Position;

#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self {
            start,
            end
        }
    }
}
