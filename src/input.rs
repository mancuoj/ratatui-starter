use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

use crate::app::{App, Focus, Msg, Overlay};

pub fn translate(app: &App, key: KeyEvent) -> Option<Msg> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    if ctrl && key.code == Char('c') {
        return Some(Msg::Quit);
    }

    if app.overlay != Overlay::None {
        return overlay_key(key);
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

fn overlay_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        Char('j') | KeyCode::Down => Some(Msg::OverlayNext),
        Char('k') | KeyCode::Up => Some(Msg::OverlayPrev),
        KeyCode::Enter => Some(Msg::OverlayConfirm),
        KeyCode::Esc => Some(Msg::OverlayClose),
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

fn palette_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        Char('t') => Some(Msg::OpenTheme),
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
    fn keys_are_scoped_to_the_focused_pane() {
        let app = App::new();
        assert_eq!(translate(&app, key(Char('t'))), None);

        let mut app = App::new();
        app.focus = Focus::Palette;
        assert_eq!(translate(&app, key(Char('k'))), None);
        assert_eq!(translate(&app, key(Char('r'))), None);
    }

    #[test]
    fn an_open_modal_owns_the_keyboard() {
        let mut app = App::new();
        app.update(Msg::OpenTheme);

        assert_eq!(translate(&app, key(Char('j'))), Some(Msg::OverlayNext));
        assert_eq!(translate(&app, key(KeyCode::Up)), Some(Msg::OverlayPrev));
        assert_eq!(
            translate(&app, key(KeyCode::Enter)),
            Some(Msg::OverlayConfirm)
        );
        assert_eq!(translate(&app, key(KeyCode::Esc)), Some(Msg::OverlayClose));

        assert_eq!(translate(&app, key(KeyCode::Tab)), None);
        assert_eq!(translate(&app, key(Char('r'))), None);
        assert_eq!(translate(&app, key(Char('t'))), None);

        assert_eq!(translate(&app, ctrl('c')), Some(Msg::Quit));
    }
}
