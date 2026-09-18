use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Focus {
    #[default]
    Counter,
    Palette,
}

impl Focus {
    pub const ALL: [Focus; 2] = [Focus::Counter, Focus::Palette];

    pub fn idx(self) -> usize {
        Self::ALL.iter().position(|f| *f == self).unwrap_or(0)
    }

    pub fn next(self) -> Self {
        Self::ALL[(self.idx() + 1) % Self::ALL.len()]
    }

    pub fn prev(self) -> Self {
        Self::ALL[(self.idx() + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Overlay {
    #[default]
    None,
    Theme {
        original: Theme,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Msg {
    Quit,
    Tick,
    FocusNext,
    FocusPrev,
    OpenTheme,
    OverlayNext,
    OverlayPrev,
    OverlayConfirm,
    OverlayClose,
    Increment,
    Decrement,
    Reset,
}

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub tick: u64,
    pub focus: Focus,
    pub overlay: Overlay,
    pub counter: i64,
    pub theme: Theme,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => self.should_quit = true,
            Msg::Tick => self.tick += 1,
            Msg::FocusNext => self.focus = self.focus.next(),
            Msg::FocusPrev => self.focus = self.focus.prev(),
            Msg::OpenTheme => {
                self.overlay = Overlay::Theme {
                    original: self.theme,
                }
            }
            Msg::OverlayNext => {
                if matches!(self.overlay, Overlay::Theme { .. }) {
                    self.theme = self.theme.next();
                }
            }
            Msg::OverlayPrev => {
                if matches!(self.overlay, Overlay::Theme { .. }) {
                    self.theme = self.theme.prev();
                }
            }
            Msg::OverlayConfirm => self.overlay = Overlay::None,
            Msg::OverlayClose => {
                if let Overlay::Theme { original } = self.overlay {
                    self.theme = original;
                }
                self.overlay = Overlay::None;
            }
            Msg::Increment => self.counter += 1,
            Msg::Decrement => self.counter -= 1,
            Msg::Reset => self.counter = 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_applies_messages() {
        let mut app = App::new();

        app.update(Msg::Quit);
        assert!(app.should_quit);

        app.update(Msg::Tick);
        app.update(Msg::Tick);
        assert_eq!(app.tick, 2);

        app.update(Msg::Increment);
        app.update(Msg::Increment);
        app.update(Msg::Decrement);
        assert_eq!(app.counter, 1);

        app.update(Msg::Reset);
        assert_eq!(app.counter, 0);
    }

    #[test]
    fn focus_cycles_between_panes() {
        let mut app = App::new();
        assert_eq!(app.focus, Focus::Counter);

        app.update(Msg::FocusNext);
        assert_eq!(app.focus, Focus::Palette);

        app.update(Msg::FocusNext);
        assert_eq!(app.focus, Focus::Counter);

        app.update(Msg::FocusPrev);
        assert_eq!(app.focus, Focus::Palette);
    }

    #[test]
    fn theme_overlay_previews_while_moving() {
        let mut app = App::new();
        app.update(Msg::OpenTheme);
        assert_eq!(
            app.overlay,
            Overlay::Theme {
                original: Theme::System
            }
        );

        app.update(Msg::OverlayNext);
        app.update(Msg::OverlayNext);
        assert_eq!(app.theme, Theme::FlexokiLight, "moving previews");

        app.update(Msg::OverlayConfirm);
        assert_eq!(app.theme, Theme::FlexokiLight);
        assert_eq!(app.overlay, Overlay::None);
    }

    #[test]
    fn closing_the_theme_overlay_restores_the_original() {
        let mut app = App::new();
        app.update(Msg::OpenTheme);
        app.update(Msg::OverlayNext);
        app.update(Msg::OverlayPrev);
        app.update(Msg::OverlayClose);

        assert_eq!(app.theme, Theme::System);
        assert_eq!(app.overlay, Overlay::None);
    }
}
