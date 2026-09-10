use ratatui::style::Color;

pub struct Palette {
    pub bg: Color,
    pub fg: Color,
    pub muted: Color,
    pub hint: Color,
    pub sel_bg: Color,
    pub accent: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
}

pub enum Theme {
    System,
    TokyoNight,
}
