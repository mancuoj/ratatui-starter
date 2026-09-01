use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEvent};

use crate::msg::Msg;

pub fn handle_event() -> Result<Option<Msg>> {
    if event::poll(Duration::from_millis(250))?
        && let Some(key) = event::read()?.as_key_press_event()
    {
        return Ok(handle_key(key));
    }
    Ok(None)
}

fn handle_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Some(Msg::Quit),
        KeyCode::Char('j') | KeyCode::Up => Some(Msg::Increment),
        KeyCode::Char('k') | KeyCode::Down => Some(Msg::Decrement),
        KeyCode::Char('r') => Some(Msg::Reset),
        _ => None,
    }
}
