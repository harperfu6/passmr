use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;

use crate::ui::ui;

pub enum InputMode {
    Normal,
    Search,
    AddKey,
    AddValue,
}

/// holds the state of the application
pub struct App {
    /// current value of the input
    pub search_input: String,
    /// current value of the key
    pub key_input: String,
    /// current value of the value
    pub value_input: String,
    /// cursor position in the input
    pub cursor_position: usize,
    /// current mode of the application
    pub mode: InputMode,
}

impl Default for App {
    fn default() -> Self {
        Self {
            search_input: String::new(),
            key_input: String::new(),
            value_input: String::new(),
            cursor_position: 0,
            mode: InputMode::Normal,
        }
    }
}

impl App {
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

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, &app))?;

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
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
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
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    _ => {}
                },
            }
        }
    }
}
