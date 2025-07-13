use std::collections::HashMap;
use std::path::PathBuf;

use buffer::buffer::Buffer;
use utils::{Position, Range};

use crate::cursor::Cursor;
use crate::errors::EditorError;

use uuid::Uuid;
use crossterm::event::{KeyCode};

#[derive(Debug)]
pub enum EditorMode {
    Normal,
    Insert,
    Visual,
    Command
}

impl std::fmt::Display for EditorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorMode::Normal => write!(f, "NORMAL"),
            EditorMode::Insert => write!(f, "INSERT"),
            EditorMode::Visual => write!(f, "VISUAL"),
            EditorMode::Command => write!(f, "COMMAND"),
        }
    }
}

#[derive(Debug)]
pub struct Editor {
    pub buffers: HashMap<Uuid, Buffer>, 
    pub buffer_order: Vec<Uuid>,
    pub buffer_cursor_pos: HashMap<Uuid, Position>,
    pub current_buffer: Option<Uuid>,
    pub cursor: Cursor, 
    pub mode: EditorMode
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            buffer_order: vec![],
            buffer_cursor_pos: HashMap::new(),
            current_buffer: None,
            cursor: Cursor::new(Position::new(0, 0)),
            mode: EditorMode::Normal
        }
    }

    pub fn create_empty_buffer(&mut self) {
        let id = Uuid::new_v4();
        let buffer = Buffer::new();

        self.buffers.insert(id, buffer);
        self.buffer_order.push(id);
        self.buffer_cursor_pos.insert(id, self.cursor.pos);

        if self.current_buffer.is_none() {
            self.current_buffer = Some(id);
        }
    }

    pub fn create_buffer_from_file(&mut self, path: PathBuf) {
        let id = Uuid::new_v4();
        let buffer = Buffer::from_file(&path);

        self.buffers.insert(id, buffer);
        self.buffer_order.push(id);
        self.buffer_cursor_pos.insert(id, self.cursor.pos);

        if self.current_buffer.is_none() {
            self.current_buffer = Some(id);
        }
    }

    pub fn save_buffer(&mut self) -> Result<(), EditorError> {
        let buffer = self.get_current_buffer_mut().unwrap();

        match buffer.save_to_file() {
            Ok(_) => Ok(()),
            Err(_) => Err(EditorError::SaveError) 
        }
    }

    pub fn get_current_buffer(&self) -> Option<&Buffer> {
        self.current_buffer.and_then(|id| self.buffers.get(&id))
    }

    pub fn get_current_buffer_mut(&mut self) -> Option<&mut Buffer> {
        self.current_buffer.and_then(|id| self.buffers.get_mut(&id))
    }

    pub fn get_buffer_display_name(&self, buffer_id: &Uuid) -> String {
        if let Some(buffer) = self.buffers.get(buffer_id) {
            match &buffer.get_path() {
                Some(path) => path.file_name()
                    .unwrap_or_else(|| path.as_os_str())
                    .to_string_lossy()
                    .into_owned(),
                None => "Untitled".to_string(),
            }
        } else {
            "Unknown".to_string()
        }
    }

    pub fn get_current_buffer_index(&self) -> usize {
        if let Some(current_id) = self.current_buffer {
            self.buffer_order.iter().position(|&id| id == current_id).unwrap_or(0)
        } else {
            0
        }
    }

    pub fn next_buffer(&mut self) {
        if let Some(key) = self.buffer_cursor_pos.get_mut(&self.current_buffer.unwrap()) {
            *key = self.cursor.pos;
        }

        if !self.buffer_order.is_empty() {
            let current_idx = self.get_current_buffer_index();
            let next_idx = (current_idx + 1) % self.buffer_order.len();

            self.current_buffer = Some(self.buffer_order[next_idx]);
            self.cursor.pos = *self.buffer_cursor_pos.get(&self.current_buffer.unwrap()).unwrap();
        }
    }

    pub fn change_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn handle_normal_mode_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('i') => self.change_mode(EditorMode::Insert),
            KeyCode::Char('v') => self.change_mode(EditorMode::Visual),
            KeyCode::Char(':') => self.change_mode(EditorMode::Command),
            KeyCode::Char('h') => self.move_cursor_left(),
            KeyCode::Char('j') => self.move_cursor_down(),
            KeyCode::Char('k') => self.move_cursor_up(),
            KeyCode::Char('l') => self.move_cursor_right(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Tab => self.next_buffer(),
            _ => {} 
        }
    }

    pub fn handle_insert_mode_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.change_mode(EditorMode::Normal),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Enter => self.newline(),
            KeyCode::Char(c) => {
                let pos = self.cursor.pos;
                if let Some(buffer) = self.get_current_buffer_mut() {
                    match buffer.insert(pos, &c.to_string()) {
                        Ok(_) => self.move_cursor_right(),
                        Err(_) => {}
                    }
                }
            },
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Right => self.move_cursor_right(),
            _ => {}
        }
    }

    pub fn handle_visual_mode_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.change_mode(EditorMode::Normal),
            _ => {}
        }
    }

    pub fn handle_command_mode_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.change_mode(EditorMode::Normal),
            _ => {}
        }
    }

    pub fn backspace(&mut self) {
        let pos = self.cursor.pos;

        if pos.column == 0 && pos.line == 0 {
            return;
        }

        if let Some(buffer) = self.get_current_buffer_mut() {
            if pos.column == 0 {
                if let Ok(current_line) = buffer.get_line(pos.line) {
                    let current_line_content = current_line.clone();

                    if current_line_content.is_empty() {
                        buffer.lines.remove(pos.line);

                        if let Ok(prev_line) = buffer.get_line(pos.line - 1) {
                            self.move_cursor_to(Position::new(pos.line - 1, prev_line.len()));
                        }
                    } else {
                        if let Ok(prev_line) = buffer.get_line(pos.line - 1) {
                            let prev_line_len = prev_line.len();
                            let joined = prev_line.clone() + &current_line_content;

                            buffer.lines[pos.line - 1] = joined;
                            buffer.lines.remove(pos.line);

                            self.move_cursor_to(Position::new(pos.line - 1, prev_line_len));
                        }
                    }
                }
            } else {
                let _ = buffer.delete(Range::new(
                    Position::new(pos.line, pos.column - 1),
                    Position::new(pos.line, pos.column - 1)
                ));

                self.move_cursor_left();
            }
        }
    }

    pub fn newline(&mut self) {
        let pos = self.cursor.pos;

        if let Some(buffer) = self.get_current_buffer_mut() {
            match buffer.insert(pos, "\n") {
                Ok(_) => self.move_cursor_to(Position::new(pos.line + 1, 0)),
                Err(_) => {}
            }
        }
    }

    pub fn move_cursor_to(&mut self, pos: Position) {
        if let Some(buffer) = self.get_current_buffer() {
            match buffer.validate_position(pos) {
                Ok(_) => self.cursor.pos = pos,
                Err(_) => {}
            }
        }
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

                    self.cursor.pos.line += 1;
                }
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

                self.cursor.pos.line -= 1;
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        if let Some(buffer) = self.get_current_buffer() {
            if let Ok(line) = buffer.get_line(self.cursor.pos.line) {
                if self.cursor.pos.column == line.len() && self.cursor.pos.line < buffer.len() {
                    if buffer.get_line(self.cursor.pos.line + 1).is_ok() {
                        self.cursor.pos.line += 1;
                        self.cursor.pos.column = 0;
                    }
                } else if self.cursor.pos.column < line.len() {
                    self.cursor.pos.column += 1;
                } 
            }
        }
    }
} 
