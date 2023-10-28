use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use ratatui::widgets::*;

struct App {}

impl App {}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                crossterm::event::KeyCode::Char('q') => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}

fn ui(frame: &mut Frame) {
    let mut text = Text::from(Line::from("Hello World!"));
    let style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::ITALIC);
    text.patch_style(style);
    let message = Paragraph::new(text);
    frame.render_widget(message, Rect::new(0, 0, 20, 20));
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run_app(&mut terminal)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
