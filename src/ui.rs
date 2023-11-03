use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::{App, InputMode};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(3),
        ])
        .split(frame.size());

    // mode text
    let mode_text = match app.mode {
        InputMode::Normal => "Normal / press 'a' to add, 's' to search, 'q' to quit",
        InputMode::Search => "Search / press 'Esc' to exit search",
        InputMode::AddKey | InputMode::AddValue => "press 'Esc' to exit add",
    };
    let mut text = Text::from(Line::from(mode_text));
    let style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::ITALIC);
    text.patch_style(style);
    let message = Paragraph::new(text);
    frame.render_widget(message, chunks[0]);

    if let InputMode::Search = app.mode {
        // search input
        let search = Paragraph::new(app.search_input.as_str())
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Search"));
        frame.render_widget(search, chunks[1]);
        frame.set_cursor(
            chunks[1].x + app.cursor_position as u16 + 1,
            chunks[1].y + 1,
        );
        let list = Paragraph::new("")
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("List"));
        frame.render_widget(list, chunks[2]);
    }

    if let InputMode::AddKey = app.mode {
        let key = Paragraph::new(app.key_input.as_str())
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Key"));
        frame.render_widget(key, chunks[1]);
        frame.set_cursor(
            chunks[1].x + app.cursor_position as u16 + 1,
            chunks[1].y + 1,
        );
    }

    if let InputMode::AddValue = app.mode {
        let key = Paragraph::new(app.key_input.as_str())
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Key"));
        frame.render_widget(key, chunks[1]);

        let value = Paragraph::new(app.value_input.as_str())
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Value"));
        frame.render_widget(value, chunks[2]);
        frame.set_cursor(
            chunks[2].x + app.cursor_position as u16 + 1,
            chunks[2].y + 1,
        );
    }
}
