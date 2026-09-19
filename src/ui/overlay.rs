use ratatui::{
    Frame,
    layout::Constraint,
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Clear, Padding, Paragraph},
};

use crate::theme::{Palette, Theme};
use crate::ui::CIRCLE;
use crate::{app::App, model::Overlay};

pub fn render(f: &mut Frame, app: &App, p: Palette) {
    match app.overlay {
        Some(Overlay::Theme { original }) => render_theme(f, p, app.theme, original),
        None => {}
    }
}

fn render_theme(f: &mut Frame, p: Palette, preview: Theme, original: Theme) {
    let area = f.area().centered(
        Constraint::Length(50),
        Constraint::Length(Theme::ALL.len() as u16 + 2),
    );

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(p.border(true))
        .title(Line::styled(" Theme ", p.title(true)))
        .style(p.base())
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    for (row, theme) in inner.rows().zip(Theme::ALL) {
        let line = if theme == original {
            Line::styled(
                format!(" {CIRCLE} {}", theme.name()),
                Style::new().fg(p.accent),
            )
        } else {
            Line::raw(format!("   {}", theme.name()))
        };
        f.render_widget(Paragraph::new(line).style(p.sel(theme == preview)), row);
    }
}
