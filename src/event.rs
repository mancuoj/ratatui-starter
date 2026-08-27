use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::{App, Message, Modal};

pub fn handle_event(app: &App) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        return Ok(handle_key(app, key));
    }
    Ok(None)
}

fn handle_key(app: &App, key: KeyEvent) -> Option<Message> {
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return Some(Message::Quit);
    }

    match app.modal {
        Modal::Add => match key.code {
            KeyCode::Enter => Some(Message::SubmitAdd),
            KeyCode::Esc => Some(Message::CancelModal),
            KeyCode::Backspace => Some(Message::Backspace),
            KeyCode::Char(c) => Some(Message::Input(c)),
            _ => None,
        },
        Modal::ConfirmDelete { .. } => match key.code {
            KeyCode::Char('y') | KeyCode::Enter => Some(Message::ConfirmDelete),
            KeyCode::Char('n') | KeyCode::Esc => Some(Message::CancelModal),
            _ => None,
        },
        Modal::None => match key.code {
            KeyCode::Char('j') | KeyCode::Down => Some(Message::MoveDown),
            KeyCode::Char('k') | KeyCode::Up => Some(Message::MoveUp),
            KeyCode::Char(' ') => Some(Message::Toggle),
            KeyCode::Char('a') | KeyCode::Enter => Some(Message::OpenAdd),
            KeyCode::Char('d') | KeyCode::Delete => Some(Message::OpenDelete),
            KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),
            _ => None,
        },
    }
}
