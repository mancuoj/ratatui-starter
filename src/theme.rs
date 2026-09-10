use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    pub bg: Color,
    pub fg: Color,
    pub muted: Color,
    pub accent: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
}

impl Palette {
    pub const SYSTEM: Palette = Palette {
        bg: Color::Reset,
        fg: Color::Reset,
        muted: Color::DarkGray,
        accent: Color::Cyan,
        success: Color::Green,
        error: Color::Red,
        warning: Color::Yellow,
        info: Color::Blue,
    };

    pub const TOKYO_NIGHT: Palette = Palette {
        bg: Color::Rgb(0x1a, 0x1b, 0x26),
        fg: Color::Rgb(0xc0, 0xca, 0xf5),
        muted: Color::Rgb(0x56, 0x5f, 0x89),
        accent: Color::Rgb(0x7a, 0xa2, 0xf7),
        success: Color::Rgb(0x9e, 0xce, 0x6a),
        error: Color::Rgb(0xf7, 0x76, 0x8e),
        warning: Color::Rgb(0xe0, 0xaf, 0x68),
        info: Color::Rgb(0x7d, 0xcf, 0xff),
    };

    pub const FLEXOKI_LIGHT: Palette = Palette {
        bg: Color::Rgb(0xff, 0xfc, 0xf0),
        fg: Color::Rgb(0x10, 0x0f, 0x0f),
        muted: Color::Rgb(0x6f, 0x6e, 0x69),
        accent: Color::Rgb(0x24, 0x83, 0x7b),
        success: Color::Rgb(0x66, 0x80, 0x0b),
        error: Color::Rgb(0xaf, 0x30, 0x29),
        warning: Color::Rgb(0xad, 0x83, 0x01),
        info: Color::Rgb(0x20, 0x5e, 0xa6),
    };

    pub fn base(self) -> Style {
        Style::new().bg(self.bg).fg(self.fg)
    }

    pub fn fg(self) -> Style {
        Style::new().fg(self.fg)
    }

    pub fn muted(self) -> Style {
        Style::new().fg(self.muted)
    }

    pub fn accent(self) -> Style {
        Style::new().fg(self.accent)
    }

    pub fn success(self) -> Style {
        Style::new().fg(self.success)
    }

    pub fn error(self) -> Style {
        Style::new().fg(self.error)
    }

    pub fn warning(self) -> Style {
        Style::new().fg(self.warning)
    }

    pub fn info(self) -> Style {
        Style::new().fg(self.info)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    System,
    TokyoNight,
    FlexokiLight,
}

impl Theme {
    pub const ALL: [Theme; 3] = [Theme::System, Theme::TokyoNight, Theme::FlexokiLight];

    pub fn name(self) -> &'static str {
        match self {
            Theme::System => "System",
            Theme::TokyoNight => "Tokyo Night",
            Theme::FlexokiLight => "Flexoki Light",
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
            Theme::FlexokiLight => Palette::FLEXOKI_LIGHT,
        }
    }
}
