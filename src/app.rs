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
