use ratatui::{
    layout::{self, Constraint, Direction, Layout, Alignment},
    style::{Modifier, Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

use editor::editor::Editor;

pub fn ui(frame: &mut Frame, editor: &Editor) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(frame.area());

    let tab_titles: Vec<String> = editor.buffer_order
        .iter()
        .map(|id| editor.get_buffer_display_name(id))
        .collect();

    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Files"))
        .select(editor.get_current_buffer_index())
        .style(Style::default().fg(Color::Yellow))
        .highlight_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));

    frame.render_widget(tabs, chunks[0]);

    let editor_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(1)
        ])
        .split(chunks[1]);

    let content = if let Some(buffer) = editor.get_current_buffer() {
        let line_numbers: Vec<Line> = buffer.lines 
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let line_num = i + 1;
                
                if i == editor.cursor.pos.line {
                    Line::from(Span::styled(
                        format!("{:>4}", line_num), 
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    ))
                } else {
                    Line::from(Span::styled(
                        format!("{:>4}", line_num),
                        Style::default().fg(Color::Rgb(150, 150, 150))
                    ))
                }
            })
            .collect();

        let line_numbers_column = Paragraph::new(line_numbers)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Right);
        
        frame.render_widget(line_numbers_column, editor_chunks[0]);

        let lines: Vec<Line> = buffer.lines
            .iter()
            .enumerate()
            .map(|(line_num, line)| {
                if line_num == editor.cursor.pos.line {
                    Line::from(Span::styled(
                        line.clone(),
                        Style::default().bg(Color::Rgb(50, 50, 50))
                    ))
                } else {
                    Line::from(line.clone())
                }
            })
            .collect();

        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("paw :3"))
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
    } else {
        Paragraph::new("No buffer open")
            .block(Block::default().borders(Borders::ALL).title("paw :3"))
            .style(Style::default().fg(Color::White))
    };

    frame.render_widget(content, editor_chunks[1]);

    if let Some(buffer) = editor.get_current_buffer() {
        let cursor_x = editor_chunks[1].x + 1 + editor.cursor.pos.column as u16;
        let cursor_y = editor_chunks[1].y + 1 + editor.cursor.pos.line as u16;
        
        if cursor_x < editor_chunks[1].x + editor_chunks[1].width - 1 && cursor_y < editor_chunks[1].y + editor_chunks[1].height - 1 {
            frame.set_cursor(cursor_x, cursor_y);
        }
    }

    let mode_text = format!("-- {} --", editor.mode);
    
    let status_text = if let Some(buffer) = editor.get_current_buffer() {
        let cursor_info = format!("{}:{}", editor.cursor.pos.line + 1, editor.cursor.pos.column + 1);
        let file_name = buffer.get_path()
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string());
        
        format!("{} | {} | {}", mode_text, file_name, cursor_info)
    } else {
        mode_text
    };

    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    frame.render_widget(status, chunks[2]);
}
