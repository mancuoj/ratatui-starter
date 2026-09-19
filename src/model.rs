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
    // overlay
    Open(Overlay),
    Confirm,
    Cancel,
    // overlay - theme
    ThemeNext,
    ThemePrev,
    // counter
    Increment,
    Decrement,
    Reset,
}
