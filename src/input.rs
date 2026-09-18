use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

use crate::app::{App, Focus, Msg};

pub fn translate(app: &App, key: KeyEvent) -> Option<Msg> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    if ctrl && key.code == Char('c') {
        return Some(Msg::Quit);
    }

    match key.code {
        Char('q') => return Some(Msg::Quit),
        KeyCode::Tab => return Some(Msg::FocusNext),
        KeyCode::BackTab => return Some(Msg::FocusPrev),
        _ => {}
    }

    match app.focus {
        Focus::Counter => counter_key(key),
        Focus::Palette => palette_key(key),
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

fn palette_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        Char('t') => Some(Msg::NextTheme),
        Char('T') => Some(Msg::PrevTheme),
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
    fn counter_focus_maps_counter_keys() {
        let app = App::new();
        assert_eq!(translate(&app, key(Char('k'))), Some(Msg::Increment));
        assert_eq!(translate(&app, key(KeyCode::Up)), Some(Msg::Increment));
        assert_eq!(translate(&app, key(Char('j'))), Some(Msg::Decrement));
        assert_eq!(translate(&app, key(KeyCode::Left)), Some(Msg::Decrement));
        assert_eq!(translate(&app, key(Char('r'))), Some(Msg::Reset));
    }

    #[test]
    fn palette_focus_maps_theme_keys() {
        let mut app = App::new();
        app.focus = Focus::Palette;
        assert_eq!(translate(&app, key(Char('t'))), Some(Msg::NextTheme));
        assert_eq!(translate(&app, key(Char('T'))), Some(Msg::PrevTheme));
    }

    #[test]
    fn keys_are_scoped_to_the_focused_pane() {
        let app = App::new();
        assert_eq!(translate(&app, key(Char('t'))), None);
        assert_eq!(translate(&app, key(Char('T'))), None);

        let mut app = App::new();
        app.focus = Focus::Palette;
        assert_eq!(translate(&app, key(Char('k'))), None);
        assert_eq!(translate(&app, key(Char('r'))), None);
    }

    #[test]
    fn global_keys_work_in_every_focus() {
        for focus in Focus::ALL {
            let mut app = App::new();
            app.focus = focus;
            assert_eq!(translate(&app, ctrl('c')), Some(Msg::Quit));
            assert_eq!(translate(&app, key(Char('q'))), Some(Msg::Quit));
            assert_eq!(translate(&app, key(KeyCode::Tab)), Some(Msg::FocusNext));
            assert_eq!(translate(&app, key(KeyCode::BackTab)), Some(Msg::FocusPrev));
            assert_eq!(translate(&app, key(Char('x'))), None);
        }
    }
}
