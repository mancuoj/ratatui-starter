use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    pub bg: Color,
    pub fg: Color,
    pub muted: Color,
    pub border: Color,
    pub accent: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
    pub sel_bg: Color,
}

impl Palette {
    pub const SYSTEM: Palette = Palette {
        bg: Color::Reset,
        fg: Color::Reset,
        muted: Color::Gray,
        border: Color::DarkGray,
        accent: Color::Cyan,
        success: Color::Green,
        error: Color::Red,
        warning: Color::Yellow,
        info: Color::Blue,
        sel_bg: Color::DarkGray,
    };

    pub const TOKYO_NIGHT: Palette = Palette {
        bg: Color::Rgb(0x1a, 0x1b, 0x26),
        fg: Color::Rgb(0xc0, 0xca, 0xf5),
        muted: Color::Rgb(0x56, 0x5f, 0x89),
        border: Color::Rgb(0x3b, 0x42, 0x61), // fg_gutter
        accent: Color::Rgb(0x7a, 0xa2, 0xf7),
        success: Color::Rgb(0x9e, 0xce, 0x6a),
        error: Color::Rgb(0xf7, 0x76, 0x8e),
        warning: Color::Rgb(0xe0, 0xaf, 0x68),
        info: Color::Rgb(0x7d, 0xcf, 0xff),
        sel_bg: Color::Rgb(0x28, 0x34, 0x57), // bg_visual: blend(blue0, 40%, bg)
    };

    pub const GRUVBOX: Palette = Palette {
        bg: Color::Rgb(0x28, 0x28, 0x28),
        fg: Color::Rgb(0xeb, 0xdb, 0xb2),
        muted: Color::Rgb(0x92, 0x83, 0x74),
        border: Color::Rgb(0x3c, 0x38, 0x36),
        accent: Color::Rgb(0xd7, 0x99, 0x21),
        success: Color::Rgb(0xb8, 0xbb, 0x26),
        error: Color::Rgb(0xfb, 0x49, 0x34),
        warning: Color::Rgb(0xfa, 0xbd, 0x2f),
        info: Color::Rgb(0x45, 0x85, 0x88),
        sel_bg: Color::Rgb(0x50, 0x49, 0x45),
    };

    pub const DRACULA: Palette = Palette {
        bg: Color::Rgb(0x28, 0x2a, 0x36),
        fg: Color::Rgb(0xf8, 0xf8, 0xf2),
        muted: Color::Rgb(0x62, 0x72, 0xa4),
        border: Color::Rgb(0x44, 0x47, 0x5a),
        accent: Color::Rgb(0xbd, 0x93, 0xf9),
        success: Color::Rgb(0x50, 0xfa, 0x7b),
        error: Color::Rgb(0xff, 0x55, 0x55),
        warning: Color::Rgb(0xf1, 0xfa, 0x8c),
        info: Color::Rgb(0x8b, 0xe9, 0xfd),
        sel_bg: Color::Rgb(0x44, 0x47, 0x5a),
    };

    pub fn base(self) -> Style {
        Style::new().bg(self.bg).fg(self.fg)
    }

    pub fn sel(self) -> Style {
        Style::new().bg(self.sel_bg).fg(self.fg)
    }

    pub fn border(self, focus: bool) -> Style {
        Style::new().fg(if focus { self.accent } else { self.border })
    }

    pub fn title(self, focus: bool) -> Style {
        if focus {
            Style::new().fg(self.accent).bold()
        } else {
            Style::new().fg(self.muted)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    System,
    TokyoNight,
    Gruvbox,
    Dracula,
}

impl Theme {
    pub const ALL: [Theme; 4] = [
        Theme::System,
        Theme::TokyoNight,
        Theme::Gruvbox,
        Theme::Dracula,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Theme::System => "System",
            Theme::TokyoNight => "Tokyo Night",
            Theme::Gruvbox => "Gruvbox",
            Theme::Dracula => "Dracula",
        }
    }

    pub fn idx(self) -> usize {
        Self::ALL.iter().position(|t| *t == self).unwrap_or(0)
    }

    pub fn next(self) -> Self {
        Self::ALL[(self.idx() + 1) % Self::ALL.len()]
    }

    pub fn prev(self) -> Self {
        Self::ALL[(self.idx() + Self::ALL.len() - 1) % Self::ALL.len()]
    }

    pub const fn palette(self) -> Palette {
        match self {
            Theme::System => Palette::SYSTEM,
            Theme::TokyoNight => Palette::TOKYO_NIGHT,
            Theme::Gruvbox => Palette::GRUVBOX,
            Theme::Dracula => Palette::DRACULA,
        }
    }
}
