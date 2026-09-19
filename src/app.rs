use crate::{
    model::{Focus, Msg, Overlay},
    theme::Theme,
};

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub tick: u64,
    pub focus: Focus,
    pub overlay: Option<Overlay>,
    pub theme: Theme,
    pub counter: i64,
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
            // overlay
            Msg::Open(overlay) => self.overlay = Some(overlay),
            Msg::Confirm => self.overlay = None,
            Msg::Cancel => {
                match self.overlay {
                    Some(Overlay::Theme { original }) => self.theme = original,
                    None => {}
                }
                self.overlay = None;
            }
            // overlay - theme
            Msg::ThemeNext => {
                if matches!(self.overlay, Some(Overlay::Theme { .. })) {
                    self.theme = self.theme.next();
                }
            }
            Msg::ThemePrev => {
                if matches!(self.overlay, Some(Overlay::Theme { .. })) {
                    self.theme = self.theme.prev();
                }
            }
            // counter
            Msg::Increment => self.counter += 1,
            Msg::Decrement => self.counter -= 1,
            Msg::Reset => self.counter = 0,
        }
    }
}
