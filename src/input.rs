use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

use crate::app::{App, Msg};

pub fn translate(_app: &App, key: KeyEvent) -> Option<Msg> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    if ctrl && key.code == Char('c') {
        return Some(Msg::Quit);
    }

    match key.code {
        Char('q') | KeyCode::Esc => Some(Msg::Quit),
        Char('k') | KeyCode::Up | KeyCode::Right => Some(Msg::Increment),
        Char('j') | KeyCode::Down | KeyCode::Left => Some(Msg::Decrement),
        Char('r') => Some(Msg::Reset),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn ctrl(c: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
    }

    #[test]
    fn translate_maps_keys_to_msgs() {
        let app = App::new();
        assert_eq!(translate(&app, ctrl('c')), Some(Msg::Quit));
        assert_eq!(translate(&app, key(Char('q'))), Some(Msg::Quit));
        assert_eq!(translate(&app, key(Char('k'))), Some(Msg::Increment));
        assert_eq!(translate(&app, key(KeyCode::Left)), Some(Msg::Decrement));
        assert_eq!(translate(&app, key(Char('r'))), Some(Msg::Reset));
        assert_eq!(translate(&app, key(Char('x'))), None);
    }
}
