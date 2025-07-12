use std::path::PathBuf;
use std::io::{Write, BufRead, BufReader};
use std::fs::{File};
use std::fs;

use utils::{Range, Position};
use crate::errors::BufferError;

#[derive(Debug)]    
pub struct Buffer {
    lines: Vec<String>,
    file_path: Option<PathBuf>
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            file_path: None
        }
    }

    pub fn from_text(text: &str) -> Self {
        let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();

        let lines = if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        };

        Self {
            lines,
            file_path: None
        }
    }

    pub fn from_file(path: &PathBuf) -> Self {
        if !path.exists() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            } 

            let _ = File::create(path);
        }

        let file = File::open(path).expect("Error opening file");
        let buf = BufReader::new(file);
        let lines: Vec<String> = buf.lines().map(|line| line.expect("Error parsing line")).collect();

        Self {
            lines,
            file_path: Some(path.to_path_buf())
        }
    }

    pub fn save_to_file(&self) -> Result<(), BufferError> {
        if let Some(path) = &self.file_path {
            let mut file = File::create(path).map_err(BufferError::IoError)?;
            let content = self.lines.join("\n");
            file.write_all(content.as_bytes()).map_err(BufferError::IoError)?;

            Ok(())
        } else {
            Err(BufferError::FileNotSet)
        }
    }

    pub fn insert(&mut self, pos: Position, text: &str) -> Result<(), BufferError> {
        self.validate_position(pos)?;

        let lines: Vec<&str> = text.lines().collect();
        if lines.is_empty() {
            return Ok(());
        }

        if lines.len() == 1 {
            self.lines[pos.line].insert_str(pos.column, text);
        } else {
            let current = &self.lines[pos.line].clone();
            let before = current[.. pos.column].to_string();
            let after = current[pos.column ..].to_string();

            self.lines[pos.line] = before + lines[0];

            for i in 1 .. lines.len() {
                self.lines.insert(pos.line + i, lines[i].to_string());
            }

            let last_line_idx = pos.line + lines.len() - 1;
            self.lines[last_line_idx].push_str(&after);
        }

        Ok(())
    }

    pub fn delete(&mut self, range: Range) -> Result<String, BufferError> {
        self.validate_position(range.start)?;
        self.validate_position(range.end)?;

        if range.start.line == range.end.line  {
            let line = &mut self.lines[range.start.line];
            let deleted = line[range.start.column .. range.end.column + 1].to_string();

            line.drain(range.start.column .. range.end.column + 1);

            Ok(deleted)
        } else {
            let mut deleted = String::new();
            let first_line = &mut self.lines[range.start.line];

            deleted.push_str(&first_line[range.start.column ..]);
            deleted.push('\n');

            for i in range.start.line + 1 .. range.end.line {
                deleted.push_str(&self.lines[i]);
                deleted.push('\n');
            }

            let last_line = &self.lines[range.end.line];
            deleted.push_str(&last_line[.. range.end.column]);

            let before = self.lines[range.start.line][.. range.start.column].to_string();
            let after = self.lines[range.end.line][range.end.column ..].to_string();
    
            self.lines.drain(range.start.line .. range.end.line);
            self.lines.insert(range.start.line, before + &after);

            Ok(deleted)
        }
    }

    pub fn get_text(&self, range: Range) -> Result<String, BufferError> {
        self.validate_position(range.start)?;
        self.validate_position(range.end)?;

        if range.start.line == range.end.line {
            let line = &self.lines[range.start.line];
            let result = line[range.start.column .. range.end.column + 1].to_string();

            Ok(result)
        } else {
            let mut result = String::new();

            let first_line = &self.lines[range.start.line];
            result.push_str(&first_line[range.start.column ..]);
            result.push('\n');

            for i in range.start.line + 1 .. range.end.line {
                result.push_str(&self.lines[i]);
                result.push('\n');
            }

            let last_line = &self.lines[range.end.line];
            result.push_str(&last_line[.. range.end.column]);

            Ok(result)
        }
    }

    pub fn get_buffer(&self) -> String {
        let mut result = String::new();

        if self.len() == 1 {
            result.push_str(&self.lines[0]);

            return result;
        }

        for i in 0 .. self.len() - 1 {
            result.push_str(&self.lines[i]);
            result.push('\n')
        }

        result.push_str(&self.lines[self.len() - 1]);

        result
    }

    pub fn get_line(&self, line: usize) -> Result<String, BufferError> {
        self.lines.get(line).map(|s| s.to_string()).ok_or(BufferError::InvalidPosition { line: line, column: 0})
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn validate_position(&self, pos: Position) -> Result<(), BufferError> {
        if pos.line > self.len() {
            return Err(BufferError::InvalidPosition { 
                line: pos.line, 
                column: pos.column
            });
        }

        if pos.column > self.get_line(pos.line).unwrap().len() {
            return Err(BufferError::InvalidPosition { 
                line: pos.line, 
                column: pos.column
            });
        }

        Ok(())
    }

    pub fn validate_range(&self, range: Range) -> Result<(), BufferError> {
        match self.validate_position(range.start) {
            Ok(_) => {},
            Err(_) => return Err(BufferError::InvalidRange)
        }

        match self.validate_position(range.end) {
            Ok(_) => Ok(()),
            Err(_) => return Err(BufferError::InvalidRange)
        } 
    }
}
