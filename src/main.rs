use std::io;
use std::env;

use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::{style::palette::material::ORANGE, Terminal};
use ratatui::prelude::CrosstermBackend;
use crossterm::{event::EnableMouseCapture, execute, terminal::{enable_raw_mode, EnterAlternateScreen}};
use anyhow::Result;

use editor::editor::Editor;
use tui::app::run_editor;


fn main() -> Result<()>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let file = env::args().nth(1).expect("nope");

    let mut editor = Editor::new();
    editor.create_buffer_from_file(file.into());
    let res = run_editor(&mut terminal, &mut editor);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
