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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_maps_keys_to_msgs() {
        let app = App::new();

        assert_eq!(
            translate(&app, KeyCode::Char('k').into()),
            Some(Msg::Increment)
        );
        assert_eq!(translate(&app, KeyCode::Down.into()), Some(Msg::Decrement));
        assert_eq!(translate(&app, KeyCode::Char('r').into()), Some(Msg::Reset));

        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(translate(&app, ctrl_c), Some(Msg::Quit));
        assert_eq!(translate(&app, KeyCode::Char('x').into()), None);
    }
}
