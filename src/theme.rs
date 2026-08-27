use ratatui::style::{Color, Modifier, Style};

pub const FG: Color = Color::Rgb(0xFC, 0xFC, 0xFA);
pub const PALE: Color = Color::Rgb(0xC1, 0xC0, 0xC0);
pub const DIM: Color = Color::Rgb(0x72, 0x70, 0x72);
pub const BORDER: Color = Color::Rgb(0x5B, 0x59, 0x5C);
pub const ACCENT: Color = Color::Rgb(0xFF, 0xD8, 0x66);
pub const SEL_BG: Color = Color::Rgb(0x40, 0x3E, 0x41);
pub const RED: Color = Color::Rgb(0xFF, 0x61, 0x88);
pub const GREEN: Color = Color::Rgb(0xA9, 0xDC, 0x76);

pub fn text() -> Style {
    Style::new().fg(FG)
}

pub fn secondary() -> Style {
    Style::new().fg(PALE)
}

pub fn muted() -> Style {
    Style::new().fg(DIM)
}

pub fn border() -> Style {
    Style::new().fg(BORDER)
}

pub fn accent() -> Style {
    Style::new().fg(ACCENT)
}

pub fn key() -> Style {
    accent().add_modifier(Modifier::BOLD)
}

pub fn selection() -> Style {
    Style::new().bg(SEL_BG)
}

pub fn completed() -> Style {
    muted().add_modifier(Modifier::CROSSED_OUT)
}

pub fn success() -> Style {
    Style::new().fg(GREEN)
}

pub fn danger() -> Style {
    Style::new().fg(RED)
}
