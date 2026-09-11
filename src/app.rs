use crate::theme::Theme;

#[derive(Debug, PartialEq, Eq)]
pub enum Msg {
    Quit,
    Tick,
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
    pub counter: i64,
    pub theme: Theme,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            tick: 0,
            counter: 0,
            theme: Theme::default(),
        }
    }

    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => self.should_quit = true,
            Msg::Tick => self.tick += 1,
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
}
