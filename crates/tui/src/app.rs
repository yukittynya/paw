use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use ratatui::prelude::Backend;
use ratatui::Terminal;

use std::io;

use editor::editor::{Editor, EditorMode};

use crate::ui::ui;

pub fn run_editor<B: Backend>(terminal: &mut Terminal<B>, editor: &mut Editor) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, editor))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            if key.code == KeyCode::Char('q') {
                return Ok(true);
            } 

            match editor.mode {
                EditorMode::Normal => {
                    editor.handle_normal_mode_input(key.code);
                },

                EditorMode::Insert => {
                    editor.handle_insert_mode_input(key.code);
                },

                EditorMode::Visual => {
                    editor.handle_visual_mode_input(key.code);
                },

                EditorMode::Command => {
                    editor.handle_command_mode_input(key.code);
                },
                _ => {}
            }
        }
    }
}
