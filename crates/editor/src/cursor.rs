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

    //If jumping to out of range, then put to end. Check in editor.rs move_cursor_to() then pass in
    //len(), 118G = pos::new(118, 0)
    pub fn move_to(&mut self, pos: Position) {
        self.pos = pos;
    }
}
