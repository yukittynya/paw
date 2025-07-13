use std::io;
use std::env;

use crossterm::cursor::DisableBlinking;
use crossterm::cursor::EnableBlinking;
use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use crossterm::{event::EnableMouseCapture, execute, terminal::{enable_raw_mode, EnterAlternateScreen}};
use anyhow::Result;

use editor::editor::Editor;
use tui::app::run_editor;


fn main() -> Result<()>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, DisableBlinking)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let file = env::args().nth(1).expect("nope");

    let mut editor = Editor::new();
    editor.create_buffer_from_file(file.into());
    let _res = run_editor(&mut terminal, &mut editor);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture, EnableBlinking)?;
    terminal.show_cursor()?;

    Ok(())
}
