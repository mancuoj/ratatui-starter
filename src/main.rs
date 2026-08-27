use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};

#[derive(Debug, Default)]
struct App {
    counter: i32,
    should_quit: bool,
}

#[derive(PartialEq)]
enum Message {
    Increment,
    Decrement,
    Reset,
    Quit,
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let result = run(&mut terminal, &mut app);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(app, frame))?;

        let mut current_msg = handle_event(app)?;
        while current_msg.is_some() {
            current_msg = update(app, current_msg.unwrap());
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn draw(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("Counter: {}", app.counter)),
        frame.area(),
    );
}

fn handle_event(_: &mut App) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        return Ok(handle_key(key));
    }
    Ok(None)
}

fn handle_key(key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(Message::Increment),
        KeyCode::Char('k') | KeyCode::Up => Some(Message::Decrement),
        KeyCode::Char('r') => Some(Message::Reset),
        KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Message::Quit),
        _ => None,
    }
}

fn update(app: &mut App, msg: Message) -> Option<Message> {
    match msg {
        Message::Increment => {
            app.counter += 1;
            if app.counter > 50 {
                return Some(Message::Reset);
            }
        }
        Message::Decrement => {
            app.counter -= 1;
            if app.counter < -50 {
                return Some(Message::Reset);
            }
        }
        Message::Reset => app.counter = 0,
        Message::Quit => app.should_quit = true,
    };

    None
}
