use ratatui::style::{Color, Modifier, Style};

pub const FG: Color = Color::Rgb(0xC0, 0xCA, 0xF5);
pub const PALE: Color = Color::Rgb(0x9A, 0xA5, 0xCE);
pub const DIM: Color = Color::Rgb(0x56, 0x5F, 0x89);
pub const BORDER: Color = Color::Rgb(0x3B, 0x42, 0x61);
pub const ACCENT: Color = Color::Rgb(0x7A, 0xA2, 0xF7);
pub const SEL_BG: Color = Color::Rgb(0x28, 0x34, 0x57);
pub const RED: Color = Color::Rgb(0xF7, 0x76, 0x8E);
pub const TEAL: Color = Color::Rgb(0x73, 0xDA, 0xCA);

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
    Style::new().fg(TEAL)
}

pub fn danger() -> Style {
    Style::new().fg(RED)
}
