use crossterm::{cursor::SetCursorStyle, event::{self, Event, KeyCode, KeyEventKind}, execute};
use ratatui::prelude::Backend;
use ratatui::Terminal;

use std::io;

use editor::editor::{Editor, EditorMode};

use crate::ui::ui;

pub fn run_editor<B: Backend>(terminal: &mut Terminal<B>, editor: &mut Editor) -> io::Result<bool> {
    let mut stdout = io::stdout();

    loop {
        terminal.draw(|f| ui(f, editor))?;

        match editor.mode {
            EditorMode::Normal => {
                execute!(stdout, SetCursorStyle::SteadyBlock)?;
            },

            EditorMode::Insert => {
                execute!(stdout, SetCursorStyle::SteadyBar)?;
            },

            EditorMode::Visual => {
                execute!(stdout, SetCursorStyle::SteadyBlock)?;
            },

            EditorMode::Command => {
                execute!(stdout, SetCursorStyle::SteadyBlock)?;
            }
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
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
                    if key.code == KeyCode::Esc {
                        editor.change_mode(EditorMode::Normal);
                        continue;
                    }

                    let mut command: String = String::new();

                    if let KeyCode::Char(c) = key.code {
                        command.push(c);
                    }

                    loop {
                        if let Event::Key(next_key) = event::read()? {
                            if next_key.kind == KeyEventKind::Release {
                                continue;
                            }

                            match next_key.code {
                                KeyCode::Enter => break,
                                KeyCode::Esc => {
                                    editor.change_mode(EditorMode::Normal);
                                    break;
                                }
                                KeyCode::Char(c) => {
                                    command.push(c);
                                }
                                KeyCode::Backspace => {
                                    command.pop();
                                }
                                _ => {}
                            }
                        }
                    }

                    match command.as_str() {
                        "q" => return Ok(true),
                        "w" => match editor.save_buffer() {
                            Ok(_) => {

                            }
                            Err(_) => {}
                        },
                        "wq" => match editor.save_buffer() {
                            Ok(_) => return Ok(true),
                            Err(_) => {}
                        }
                        _ => {}
                    }

                    editor.change_mode(EditorMode::Normal); 
                }
            }
        }
    }
}
