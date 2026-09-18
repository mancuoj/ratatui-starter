use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Clear, Padding, Paragraph},
};

use crate::app::{App, Overlay};
use crate::theme::{Palette, Theme};
use crate::ui::CIRCLE;

pub fn render(f: &mut Frame, app: &App, p: Palette) {
    match app.overlay {
        Overlay::None => {}
        Overlay::Theme { original } => render_theme_modal(f, app.theme, original, p),
    }
}

fn render_theme_modal(f: &mut Frame, preview: Theme, original: Theme, p: Palette) {
    let area = centered(f.area(), 50, Theme::ALL.len() as u16 + 2);
    f.render_widget(Clear, area);

    let block = modal_block("Theme", p);
    let inner = block.inner(area);
    f.render_widget(block, area);

    for (row, theme) in Theme::ALL.iter().enumerate() {
        let y = inner.y + row as u16;
        if y >= inner.bottom() {
            break;
        }
        let style = if *theme == preview { p.sel() } else { p.base() };
        let row_area = Rect {
            x: inner.x,
            y,
            width: inner.width,
            height: 1,
        };
        let line = theme_row(*theme, *theme == original, p);
        f.render_widget(Paragraph::new(line).style(style), row_area);
    }
}

fn theme_row(theme: Theme, original: bool, p: Palette) -> Line<'static> {
    let marker = if original { CIRCLE } else { " " };
    let style = if original {
        Style::new().fg(p.accent)
    } else {
        Style::new()
    };
    Line::styled(format!(" {marker} {}", theme.name()), style)
}

fn modal_block(title: &str, p: Palette) -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(p.border(true))
        .title(Line::styled(format!(" {title} "), p.title(true)))
        .style(p.base())
        .padding(Padding::horizontal(1))
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let w = width.min(area.width.saturating_sub(2));
    let h = height.min(area.height.saturating_sub(2));
    Rect {
        x: area.x + (area.width - w) / 2,
        y: area.y + (area.height - h) / 2,
        width: w,
        height: h,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Msg;
    use crate::ui;
    use ratatui::{Terminal, backend::TestBackend};

    fn screen(app: &App) -> String {
        let mut terminal = Terminal::new(TestBackend::new(50, 16)).unwrap();
        terminal.draw(|f| ui::render(f, app)).unwrap();
        let buf = terminal.backend().buffer();
        let mut out = String::new();
        for y in buf.area.y..buf.area.bottom() {
            for x in buf.area.x..buf.area.right() {
                out.push_str(buf[(x, y)].symbol());
            }
            out.push('\n');
        }
        out
    }

    #[test]
    fn theme_modal_lists_every_theme() {
        let mut app = App::new();
        app.update(Msg::OpenTheme);
        let screen = screen(&app);
        for theme in Theme::ALL {
            assert!(screen.contains(theme.name()), "missing {}", theme.name());
        }
    }

    #[test]
    fn theme_modal_dots_the_original_and_previews_the_selection() {
        let mut app = App::new();
        app.update(Msg::OpenTheme);
        app.update(Msg::OverlayNext);

        let screen = screen(&app);
        assert!(
            screen.contains(&format!("{CIRCLE} System")),
            "original keeps the dot"
        );
        assert!(
            !screen.contains(&format!("{CIRCLE} Tokyo Night")),
            "the cursor takes no dot"
        );
        assert!(
            screen.contains("PALETTE · Tokyo Night"),
            "moving previews the whole frame"
        );
    }
}
