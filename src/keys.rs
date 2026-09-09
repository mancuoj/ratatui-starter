use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Msg};

pub fn translate(_app: &App, key: KeyEvent) -> Option<Msg> {
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return Some(Msg::Quit);
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Some(Msg::Quit),
        KeyCode::Char('k') | KeyCode::Up | KeyCode::Right => Some(Msg::Increment),
        KeyCode::Char('j') | KeyCode::Down | KeyCode::Left => Some(Msg::Decrement),
        KeyCode::Char('r') => Some(Msg::Reset),
        _ => None,
    }
}
