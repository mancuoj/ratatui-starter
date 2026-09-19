use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

use crate::{
    app::App,
    model::{Focus, Msg, Overlay},
};

pub fn translate(app: &App, key: KeyEvent) -> Option<Msg> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    if ctrl && key.code == Char('c') {
        return Some(Msg::Quit);
    }

    match app.overlay {
        Some(Overlay::Theme { .. }) => return theme_key(key),
        None => {}
    };

    match key.code {
        Char('q') => return Some(Msg::Quit),
        KeyCode::Tab => return Some(Msg::FocusNext),
        KeyCode::BackTab => return Some(Msg::FocusPrev),
        _ => {}
    }

    match app.focus {
        Focus::Counter => counter_key(key),
        Focus::Palette => palette_key(app, key),
    }
}

fn theme_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        Char('j') | KeyCode::Down => Some(Msg::ThemeNext),
        Char('k') | KeyCode::Up => Some(Msg::ThemePrev),
        KeyCode::Enter => Some(Msg::Confirm),
        Char('q') | KeyCode::Esc => Some(Msg::Cancel),
        _ => None,
    }
}

fn counter_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        Char('k') | KeyCode::Up | KeyCode::Right => Some(Msg::Increment),
        Char('j') | KeyCode::Down | KeyCode::Left => Some(Msg::Decrement),
        Char('r') => Some(Msg::Reset),
        _ => None,
    }
}

fn palette_key(app: &App, key: KeyEvent) -> Option<Msg> {
    match key.code {
        Char('t') => Some(Msg::Open(Overlay::Theme {
            original: app.theme,
        })),
        _ => None,
    }
}
