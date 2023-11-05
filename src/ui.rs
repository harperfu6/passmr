use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::{App, InputMode};
use crate::kvs::Kvs;

pub fn ui(frame: &mut Frame, app: &mut App, kvs: &mut Kvs) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(30),
            Constraint::Length(1),
        ])
        .split(frame.size());

    // mode text
    let mode_text = match app.mode {
        InputMode::Normal => "Normal Mode: press 's' to search, 'a' to add, 'q' to quit",
        InputMode::Search => "Search Mode: press 'Esc' to exit search",
        InputMode::Select => "Select Mode: press 'Esc' to exit select",
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
        let search = Paragraph::new(app.search_input.as_str())
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Search"));
        frame.render_widget(search, chunks[1]);
        frame.set_cursor(
            chunks[1].x + app.cursor_position as u16 + 1,
            chunks[1].y + 1,
        );

        let key_list = app.get_target_key_list();
        let list_items = key_list
            .iter()
            .map(|i| {
                let lines = vec![Line::from(i.as_str())];
                ListItem::new(lines)
            })
            .collect::<Vec<ListItem>>();
        let ui_key_list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title("Key"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        let mut_key_list = app.get_mut_key_list();
        frame.render_stateful_widget(ui_key_list, chunks[2], &mut mut_key_list.state);
    }

    if let InputMode::Select = app.mode {
        let key_list = app.get_target_statefule_list();
        let list_items = key_list
            .items
            .iter()
            .map(|i| {
                let lines = vec![Line::from(i.as_str())];
                ListItem::new(lines)
            })
            .collect::<Vec<ListItem>>();
        let ui_key_list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title("Key"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        let mut_key_list = app.get_mut_key_list();
        frame.render_stateful_widget(ui_key_list, chunks[2], &mut mut_key_list.state);

        if let Some(key) = app.get_selected_key() {
            let value = kvs.get(key.as_str()).unwrap();
            let value = Paragraph::new(value.as_str())
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("Value"));
            frame.render_widget(value, chunks[3]);
        }
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
