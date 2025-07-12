use std::collections::HashMap;
use buffer::buffer::Buffer;
use utils::Position;
use crate::cursor::Cursor;
use uuid::Uuid;

#[derive(Debug)]
pub enum EditorMode {
    Normal,
    Insert,
    Visual,
    Command
}

#[derive(Debug)]
pub struct Editor {
    buffers: HashMap<Uuid, Buffer>, 
    current_buffer: Option<Uuid>,
    cursor: Cursor, 
    mode: EditorMode
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            current_buffer: None,
            cursor: Cursor::new(Position::new(0, 0)),
            mode: EditorMode::Normal
        }
    }

    pub fn create_buffer(&mut self) {
        let id = Uuid::new_v4();
        let buffer = Buffer::new();

        self.buffers.insert(id, buffer);

        if self.current_buffer.is_none() {
            self.current_buffer = Some(id);
        }
    }

    pub fn get_current_buffer(&self) -> Option<&Buffer> {
        self.current_buffer.and_then(|id| self.buffers.get(&id))
    }

    pub fn get_current_buffer_mut(&mut self) -> Option<&mut Buffer> {
        self.current_buffer.and_then(|id| self.buffers.get_mut(&id))
    }

    pub fn move_cursor_left(&mut self) {
        if let Some(buffer) = self.get_current_buffer() {
            if self.cursor.pos.column == 0 && self.cursor.pos.line > 0 {
                let new_line = self.cursor.pos.line - 1;

                if let Ok(line) = buffer.get_line(new_line) {
                    self.cursor.pos.line = new_line;
                    self.cursor.pos.column = line.len();
                }
            } else if self.cursor.pos.column > 0 {
                self.cursor.pos.column -= 1;
            }
        }
    }

    pub fn move_cursor_down(&mut self) {        
        if let Some(buffer) = self.get_current_buffer() {
            if self.cursor.pos.line < buffer.len() {
                if let Ok(line) = buffer.get_line(self.cursor.pos.line + 1) {
                    if self.cursor.pos.column > line.len() {
                        self.cursor.pos.column = line.len();
                    }
                }

                self.cursor.pos.line += 1;
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if let Some(buffer) = self.get_current_buffer() {
            if self.cursor.pos.line > 0 {
                if let Ok(line) = buffer.get_line(self.cursor.pos.line - 1) {
                    if self.cursor.pos.column > line.len() {
                        self.cursor.pos.column = line.len();
                    }
                }

                self.cursor.pos.line += 1;
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        if let Some(buffer) = self.get_current_buffer() {
            if let Ok(line) = buffer.get_line(self.cursor.pos.line) {
                if self.cursor.pos.column == line.len() && self.cursor.pos.line < buffer.len() {
                    self.cursor.pos.line += 1;
                    self.cursor.pos.column = 0;
                } else if self.cursor.pos.column < line.len() {
                    self.cursor.pos.column += 1;
                } 
            }
        }
    }
} 
