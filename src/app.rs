use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::kvs::Kvs;
use crate::ui::ui;

pub enum InputMode {
    Normal,
    Search,
    Select,
    AddKey,
    AddValue,
}

#[derive(Debug, Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }
}

/// holds the state of the application
pub struct App {
    /// search input in search mode
    pub search_input: String,
    /// list of stored keys
    pub key_list: Vec<String>,
    /// list of search target keys
    pub target_key_list: StatefulList<String>,
    /// key input in add mode
    pub key_input: String,
    /// value input in add mode
    pub value_input: String,
    /// cursor position in the input
    pub cursor_position: usize,
    /// current mode of the app
    pub mode: InputMode,
}

impl App {
    pub fn new() -> Self {
        Self {
            search_input: String::new(),
            key_input: String::new(),
            key_list: vec![],
            target_key_list: StatefulList::with_items(vec![]),
            value_input: String::new(),
            cursor_position: 0,
            mode: InputMode::Normal,
        }
    }

    // search時はリストを更新し、select時は更新しないようにする

    pub fn get_target_key_list(&mut self) -> Vec<String> {
        let target_key_list: Vec<String> = self
            .key_list
            .iter()
            .filter(|key| key.contains(&self.search_input))
            .map(|key| key.to_owned())
            .collect();
        self.target_key_list = StatefulList::with_items(target_key_list);
        self.target_key_list.items.to_owned()
    }

    pub fn get_target_statefule_list(&mut self) -> StatefulList<String> {
        self.target_key_list.to_owned()
    }

    pub fn get_mut_key_list(&mut self) -> &mut StatefulList<String> {
        &mut self.target_key_list
    }

    pub fn sync_key_list(&mut self, key_list: Vec<String>) {
        self.key_list = key_list;
    }

    pub fn get_selected_key(&self) -> Option<String> {
        match self.target_key_list.state.selected() {
            Some(i) => Some(self.target_key_list.items[i].to_owned()),
            None => None,
        }
    }

    fn add_to_kvs(&mut self, kvs: &Kvs) {
        if !self.key_input.is_empty() && !self.value_input.is_empty() {
            kvs.insert(&self.key_input, &self.value_input);

            self.sync_key_list(kvs.get_key_vec());

            self.key_input.clear();
            self.value_input.clear();
            self.mode = InputMode::Normal;
        }
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor_position(cursor_moved_right);
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor_position(cursor_moved_left);
    }

    fn clamp_cursor_position(&self, new_cursor_position: usize) -> usize {
        match self.mode {
            InputMode::Search => new_cursor_position.clamp(0, self.search_input.len()),
            InputMode::AddKey => new_cursor_position.clamp(0, self.key_input.len()),
            InputMode::AddValue => new_cursor_position.clamp(0, self.value_input.len()),
            _ => 0,
        }
    }

    fn enter_char(&mut self, c: char) {
        match self.mode {
            InputMode::Search => {
                self.search_input.insert(self.cursor_position, c);
                self.move_cursor_right();
            }
            InputMode::AddKey => {
                self.key_input.insert(self.cursor_position, c);
                self.move_cursor_right();
            }
            InputMode::AddValue => {
                self.value_input.insert(self.cursor_position, c);
                self.move_cursor_right();
            }
            _ => {}
        }
    }

    fn delete_char(&mut self) {
        match self.mode {
            InputMode::Search => {
                if self.cursor_position > 0 {
                    self.search_input.remove(self.cursor_position - 1);
                    self.move_cursor_left();
                }
            }
            InputMode::AddKey => {
                if self.cursor_position > 0 {
                    self.key_input.remove(self.cursor_position - 1);
                    self.move_cursor_left();
                }
            }
            InputMode::AddValue => {
                if self.cursor_position > 0 {
                    self.value_input.remove(self.cursor_position - 1);
                    self.move_cursor_left();
                }
            }
            _ => {}
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    kvs: &mut Kvs,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app, kvs))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('a') => {
                        app.mode = InputMode::AddKey;
                        app.cursor_position = app.key_input.len();
                    }
                    KeyCode::Char('s') => {
                        app.mode = InputMode::Search;
                        app.cursor_position = app.search_input.len();
                    }
                    _ => {}
                },
                InputMode::Search => match key.code {
                    KeyCode::Esc => {
                        app.mode = InputMode::Normal;
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Enter => {
                        app.mode = InputMode::Select;
                        app.target_key_list.state.select(Some(0));
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    _ => {}
                },
                InputMode::Select => match key.code {
                    KeyCode::Esc => {
                        app.mode = InputMode::Normal;
                    }
                    KeyCode::Down => {
                        app.target_key_list.next();
                    }
                    KeyCode::Up => {
                        app.target_key_list.previous();
                    }
                    _ => {}
                },
                InputMode::AddKey => match key.code {
                    KeyCode::Esc => {
                        app.mode = InputMode::Normal;
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Enter => {
                        app.mode = InputMode::AddValue;
                        app.cursor_position = app.value_input.len();
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    _ => {}
                },
                InputMode::AddValue => match key.code {
                    KeyCode::Esc => {
                        app.mode = InputMode::Normal;
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Enter => {
                        app.add_to_kvs(kvs);
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    _ => {}
                },
            }
        }
    }
}
