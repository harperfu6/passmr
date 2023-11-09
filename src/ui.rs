use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::StatefulList;
use crate::app::{App, InputMode};
use crate::kvs::Kvs;

fn text_area(text_list: Vec<&str>, frame: &mut Frame, area: &Rect, is_warning: bool) {
    let text = text_list
        .into_iter()
        .map(|t| t.into())
        .collect::<Vec<Line>>();
    if is_warning {
        frame.render_widget(
            Paragraph::new(text)
                .style(Style::default().fg(Color::LightMagenta))
                .block(Block::default().borders(Borders::ALL)),
            *area,
        );
    } else {
        frame.render_widget(
            Paragraph::new(text).block(Block::default().borders(Borders::ALL)),
            *area,
        );
    }
}

fn str_widget_area(
    string: String,
    title: &str,
    frame: &mut Frame,
    app: &mut App,
    area: &Rect,
    is_input: bool,
) {
    let input = string.as_str();
    let search = Paragraph::new(input)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(search, *area);

    if is_input {
        frame.set_cursor(area.x + app.cursor_position as u16 + 1, area.y + 1);
    }
}

fn str_list_widget_area(
    str_list: Vec<String>,
    title: &str,
    frame: &mut Frame,
    app: &mut App,
    area: &Rect,
) {
    let list_items = str_list
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.as_str())];
            ListItem::new(lines)
        })
        .collect::<Vec<ListItem>>();
    let ui_key_list =
        List::new(list_items).block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(ui_key_list, *area);
}

fn stateful_list_widget_area(
    stateful_list: StatefulList<String>,
    title: &str,
    frame: &mut Frame,
    app: &mut App,
    area: &Rect,
) {
    let list_items = stateful_list
        .items
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.as_str())];
            ListItem::new(lines)
        })
        .collect::<Vec<ListItem>>();
    let ui_key_list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut_key_list = app.get_mut_stateful_key_list();
    frame.render_stateful_widget(ui_key_list, *area, &mut mut_key_list.state);
}

pub fn ui(frame: &mut Frame, app: &mut App, kvs: &mut Kvs) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Length(3),
            Constraint::Length(30),
            Constraint::Length(1),
        ])
        .split(frame.size());

    let mode_text = match app.mode {
        InputMode::Home => vec![
            "=========================",
            "   Welcome to passmr!   ",
            "=========================",
            "- press 's' to search",
            "- press 'a' to add",
            "- press 'q' to quit",
        ],
        InputMode::Search => vec![
            "Search Mode:",
            "- press 'Enter' to get value",
            "- press 'Esc' to exit search mode",
        ],
        InputMode::Select => vec![
            "Select Mode:",
            "- press 'Enter' to copy to clipboard",
            "- press 'j' to move down",
            "- press 'k' to move up",
            "- press 'e' to edit value",
            "- press 'd' to delete key-value",
            "- press 'Esc' to exit select mode",
        ],
        InputMode::Delete => vec!["press 'y' to delete", "press 'Esc' to cancel"],
        InputMode::Edit => vec![
            "Edit Mode:",
            "- press 'Enter' to save",
            "- press 'Esc' to exit edit mode",
        ],
        InputMode::AddKey => vec![
            "Add Key-Value Mode:",
            "- press 'Enter' to add value",
            "- press 'Esc' to exit add key-value mode",
        ],
        InputMode::AddValue => vec![
            "Add Key-Value Mode:",
            "- press 'Enter' to save",
            "- press 'Esc' to exit add key-value mode",
        ],
    };
    match app.mode {
        InputMode::Delete => text_area(mode_text, frame, &chunks[0], true),
        _ => text_area(mode_text, frame, &chunks[0], false),
    };

    match app.mode {
        InputMode::Home => {}
        InputMode::Search => {
            str_widget_area(
                app.search_input.clone(),
                "Search",
                frame,
                app,
                &chunks[1],
                true,
            );
            str_list_widget_area(app.get_search_key_list(), "Key", frame, app, &chunks[2]);
        }
        InputMode::Select => {
            stateful_list_widget_area(app.get_statefule_list(), "Key", frame, app, &chunks[2]);
            if let Some(key) = app.get_selected_key() {
                let value = kvs.get(key.as_str()).unwrap();
                str_widget_area(value.clone(), "Value", frame, app, &chunks[3], false);
            }
        }
        InputMode::Edit => {
            if let Some(key) = app.get_selected_key() {
                str_widget_area(key.clone(), "Key", frame, app, &chunks[2], false);
                str_widget_area(
                    app.value_input.clone(),
                    "Value",
                    frame,
                    app,
                    &chunks[3],
                    true,
                );
            }
        }
        InputMode::Delete => {
            if let Some(key) = app.get_selected_key() {
                str_widget_area(key.clone(), "Key", frame, app, &chunks[2], false);
                let value = kvs.get(key.as_str()).unwrap();
                str_widget_area(value.clone(), "Value", frame, app, &chunks[3], false);
            }
        }
        InputMode::AddKey => {
            str_widget_area(app.key_input.clone(), "Key", frame, app, &chunks[1], true);
        }
        InputMode::AddValue => {
            str_widget_area(app.key_input.clone(), "Key", frame, app, &chunks[1], false);
            str_widget_area(
                app.value_input.clone(),
                "Value",
                frame,
                app,
                &chunks[2],
                true,
            );
        }
    }
}
