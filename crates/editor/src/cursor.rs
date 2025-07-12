use utils::Position;

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub pos: Position
}

impl Cursor {
    pub fn new(pos: Position) -> Self {
        Self {
            pos
        }
    }
}
