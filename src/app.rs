use std::io;

use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::kvs::Kvs;
use crate::ui::ui;

pub enum InputMode {
    Home,
    Search,
    Select,
    Edit,
    Delete,
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
    pub stateful_key_list: StatefulList<String>,
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
            stateful_key_list: StatefulList::with_items(vec![]),
            value_input: String::new(),
            cursor_position: 0,
            mode: InputMode::Home,
        }
    }

    pub fn get_search_key_list(&mut self) -> Vec<String> {
        let search_key_list: Vec<String> = self
            .key_list
            .iter()
            .filter(|key| {
                let key_lower = key.to_lowercase();
                let search_input_lower = self.search_input.to_lowercase();
                key_lower.contains(&search_input_lower)
            })
            .map(|key| key.to_owned())
            .collect();
        self.stateful_key_list = StatefulList::with_items(search_key_list.clone()); // required to
                                                                                    // initialize
                                                                                    // stateful_key_list
        search_key_list
    }

    pub fn get_statefule_list(&mut self) -> StatefulList<String> {
        self.stateful_key_list.to_owned()
    }

    pub fn get_mut_stateful_key_list(&mut self) -> &mut StatefulList<String> {
        &mut self.stateful_key_list
    }

    pub fn sync_key_list(&mut self, key_list: Vec<String>) {
        self.key_list = key_list;
    }

    pub fn get_selected_key(&self) -> Option<String> {
        match self.stateful_key_list.state.selected() {
            Some(i) => Some(self.stateful_key_list.items[i].to_owned()),
            None => None,
        }
    }

    fn remove_from_kvs(&mut self, kvs: &Kvs) {
        if let Some(key) = self.get_selected_key() {
            kvs.delete(key.as_str());
            self.sync_key_list(kvs.get_key_vec());
        }
    }

    fn add_to_kvs(&mut self, kvs: &Kvs) {
        if !self.key_input.is_empty() && !self.value_input.is_empty() {
            kvs.insert(&self.key_input, &self.value_input);

            self.sync_key_list(kvs.get_key_vec());

            self.key_input.clear();
            self.value_input.clear();
            self.mode = InputMode::Home;
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
            InputMode::Edit => new_cursor_position.clamp(0, self.value_input.len()),
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
            InputMode::AddValue | InputMode::Edit => {
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
            InputMode::AddValue | InputMode::Edit => {
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
                InputMode::Home => match key.code {
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
                        app.mode = InputMode::Home;
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Enter => {
                        if app.stateful_key_list.items.len() > 0 {
                            app.mode = InputMode::Select;
                            app.stateful_key_list.state.select(Some(0));
                        }
                        app.search_input.clear();
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    _ => {}
                },
                InputMode::Select => match key.code {
                    KeyCode::Esc => {
                        app.mode = InputMode::Search;
                        app.search_input.clear();
                        app.cursor_position = 0;
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.stateful_key_list.next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.stateful_key_list.previous();
                    }
                    KeyCode::Char('d') => {
                        app.mode = InputMode::Delete;
                    }
                    KeyCode::Char('e') => {
                        if let Some(key) = app.get_selected_key() {
                            let value = kvs.get(key.as_str()).unwrap();
                            app.cursor_position = value.len();
                            app.value_input = value.to_string();
                            app.mode = InputMode::Edit;
                        }
                    }
                    KeyCode::Enter => {
                        // copy to clipboard
                        let selected_key = app.get_selected_key();
                        let value = kvs.get(&selected_key.unwrap());
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents(value.unwrap()).unwrap();
                    }
                    _ => {}
                },
                InputMode::Edit => match key.code {
                    KeyCode::Esc => {
                        app.mode = InputMode::Select;
                        app.search_input.clear();
                        app.cursor_position = 0;
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Enter => {
                        app.key_input = app.get_selected_key().unwrap(); // required to add to kvs
                        app.add_to_kvs(kvs);

                        app.mode = InputMode::Select;
                        app.search_input.clear();
                        app.cursor_position = 0;
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    _ => {}
                },
                InputMode::Delete => match key.code {
                    KeyCode::Char('y') => {
                        app.remove_from_kvs(kvs);
                        app.mode = InputMode::Search;
                        app.search_input.clear();
                        app.cursor_position = 0;
                    }
                    KeyCode::Esc => {
                        app.mode = InputMode::Select;
                    }
                    _ => {}
                },
                InputMode::AddKey => match key.code {
                    KeyCode::Esc => {
                        app.mode = InputMode::Home;
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Enter => {
                        if app.key_input.len() > 0 {
                            app.mode = InputMode::AddValue;
                            app.cursor_position = app.value_input.len();
                        }
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    _ => {}
                },
                InputMode::AddValue => match key.code {
                    KeyCode::Esc => {
                        app.mode = InputMode::AddKey;
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
