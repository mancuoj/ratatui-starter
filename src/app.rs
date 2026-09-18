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

#[derive(Debug, PartialEq, Eq)]
pub enum Msg {
    Quit,
    Tick,
    FocusNext,
    FocusPrev,
    Increment,
    Decrement,
    Reset,
    NextTheme,
    PrevTheme,
}

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub tick: u64,
    pub focus: Focus,
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
            Msg::Increment => self.counter += 1,
            Msg::Decrement => self.counter -= 1,
            Msg::Reset => self.counter = 0,
            Msg::NextTheme => self.theme = self.theme.next(),
            Msg::PrevTheme => self.theme = self.theme.prev(),
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

        app.update(Msg::NextTheme);
        assert_eq!(app.theme, Theme::TokyoNight);

        app.update(Msg::PrevTheme);
        assert_eq!(app.theme, Theme::System);
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
}
