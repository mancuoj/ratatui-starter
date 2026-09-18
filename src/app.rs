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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overlay {
    Theme { original: Theme },
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
    // overlay
    Open(Overlay),
    Confirm,
    Cancel,
    // overlay - theme
    ThemeNext,
    ThemePrev,
}

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub tick: u64,
    pub focus: Focus,
    pub overlay: Option<Overlay>,
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
            Msg::Open(overlay) => self.overlay = Some(overlay),
            Msg::Confirm => self.overlay = None,
            Msg::Cancel => {
                match self.overlay {
                    Some(Overlay::Theme { original }) => self.theme = original,
                    None => {}
                }
                self.overlay = None;
            }
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
        }
    }
}
