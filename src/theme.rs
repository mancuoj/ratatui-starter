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
    pub selection_bg: Color,
}

impl Palette {
    pub const SYSTEM: Palette = Palette {
        bg: Color::Reset,
        fg: Color::Reset,
        muted: Color::DarkGray,
        border: Color::DarkGray,
        accent: Color::Cyan,
        success: Color::Green,
        error: Color::Red,
        warning: Color::Yellow,
        info: Color::Blue,
        selection_bg: Color::DarkGray,
    };

    pub const TOKYO_NIGHT: Palette = Palette {
        bg: Color::Rgb(0x1a, 0x1b, 0x26),
        fg: Color::Rgb(0xc0, 0xca, 0xf5),
        muted: Color::Rgb(0x56, 0x5f, 0x89),
        border: Color::Rgb(0x3b, 0x42, 0x61),
        accent: Color::Rgb(0x7a, 0xa2, 0xf7),
        success: Color::Rgb(0x9e, 0xce, 0x6a),
        error: Color::Rgb(0xf7, 0x76, 0x8e),
        warning: Color::Rgb(0xe0, 0xaf, 0x68),
        info: Color::Rgb(0x7d, 0xcf, 0xff),
        selection_bg: Color::Rgb(0x28, 0x34, 0x57),
    };

    pub const FLEXOKI_LIGHT: Palette = Palette {
        bg: Color::Rgb(0xff, 0xfc, 0xf0),
        fg: Color::Rgb(0x10, 0x0f, 0x0f),
        muted: Color::Rgb(0x6f, 0x6e, 0x69),
        border: Color::Rgb(0xda, 0xd8, 0xce),
        accent: Color::Rgb(0x24, 0x83, 0x7b),
        success: Color::Rgb(0x66, 0x80, 0x0b),
        error: Color::Rgb(0xaf, 0x30, 0x29),
        warning: Color::Rgb(0xad, 0x83, 0x01),
        info: Color::Rgb(0x20, 0x5e, 0xa6),
        selection_bg: Color::Rgb(0xe6, 0xe4, 0xd9),
    };

    pub const FLEXOKI_DARK: Palette = Palette {
        bg: Color::Rgb(0x10, 0x0f, 0x0f),
        fg: Color::Rgb(0xce, 0xcd, 0xc3),
        muted: Color::Rgb(0x87, 0x85, 0x80),
        border: Color::Rgb(0x34, 0x33, 0x31),
        accent: Color::Rgb(0x3a, 0xa9, 0x9f),
        success: Color::Rgb(0x87, 0x9a, 0x39),
        error: Color::Rgb(0xd1, 0x4d, 0x41),
        warning: Color::Rgb(0xd0, 0xa2, 0x15),
        info: Color::Rgb(0x43, 0x85, 0xbe),
        selection_bg: Color::Rgb(0x28, 0x27, 0x26),
    };

    pub const ONE_LIGHT: Palette = Palette {
        bg: Color::Rgb(0xfa, 0xfa, 0xfa),
        fg: Color::Rgb(0x38, 0x3a, 0x42),
        muted: Color::Rgb(0xa0, 0xa1, 0xa7),
        border: Color::Rgb(0xd3, 0xd4, 0xd5),
        accent: Color::Rgb(0x40, 0x78, 0xf2),
        success: Color::Rgb(0x50, 0xa1, 0x4f),
        error: Color::Rgb(0xe4, 0x56, 0x49),
        warning: Color::Rgb(0xc1, 0x84, 0x01),
        info: Color::Rgb(0x01, 0x84, 0xbc),
        selection_bg: Color::Rgb(0xe5, 0xe5, 0xe6),
    };

    pub const ONE_DARK: Palette = Palette {
        bg: Color::Rgb(0x28, 0x2c, 0x34),
        fg: Color::Rgb(0xab, 0xb2, 0xbf),
        muted: Color::Rgb(0x5c, 0x63, 0x70),
        border: Color::Rgb(0x3c, 0x40, 0x49),
        accent: Color::Rgb(0x61, 0xaf, 0xef),
        success: Color::Rgb(0x98, 0xc3, 0x79),
        error: Color::Rgb(0xe0, 0x6c, 0x75),
        warning: Color::Rgb(0xe5, 0xc0, 0x7b),
        info: Color::Rgb(0x56, 0xb6, 0xc2),
        selection_bg: Color::Rgb(0x3e, 0x44, 0x51),
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
        selection_bg: Color::Rgb(0x50, 0x49, 0x45),
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
        selection_bg: Color::Rgb(0x44, 0x47, 0x5a),
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

    pub fn border(self) -> Style {
        Style::new().fg(self.border)
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

    pub fn sel(self) -> Style {
        Style::new().bg(self.selection_bg)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    System,
    TokyoNight,
    FlexokiLight,
    FlexokiDark,
    OneLight,
    OneDark,
    Gruvbox,
    Dracula,
}

impl Theme {
    pub const ALL: [Theme; 8] = [
        Theme::System,
        Theme::TokyoNight,
        Theme::FlexokiLight,
        Theme::FlexokiDark,
        Theme::OneLight,
        Theme::OneDark,
        Theme::Gruvbox,
        Theme::Dracula,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Theme::System => "System",
            Theme::TokyoNight => "Tokyo Night",
            Theme::FlexokiLight => "Flexoki Light",
            Theme::FlexokiDark => "Flexoki Dark",
            Theme::OneLight => "One Light",
            Theme::OneDark => "One Dark",
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
            Theme::FlexokiLight => Palette::FLEXOKI_LIGHT,
            Theme::FlexokiDark => Palette::FLEXOKI_DARK,
            Theme::OneLight => Palette::ONE_LIGHT,
            Theme::OneDark => Palette::ONE_DARK,
            Theme::Gruvbox => Palette::GRUVBOX,
            Theme::Dracula => Palette::DRACULA,
        }
    }
}
