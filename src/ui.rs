use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph},
};

use crate::app::App;
use crate::theme::Palette;

pub fn render(f: &mut Frame, app: &App) {
    let palette = app.theme.palette();

    f.render_widget(Block::default().style(palette.base()), f.area());

    let [_, main, footer, _] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(f.area());

    let [counter_area, palette_area] =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]).areas(main);
    render_counter(f, counter_area, app, palette);
    render_palette(f, palette_area, app, palette);

    render_footer(f, footer, palette);
}

fn render_counter(f: &mut Frame, area: Rect, app: &App, palette: Palette) {
    let content = vec![
        Line::from(vec![
            Span::raw("Counter: "),
            Span::styled(format!("{}", app.counter), palette.accent().bold()),
        ]),
        Line::raw(""),
        Line::raw("j/k    change"),
        Line::raw("r      reset"),
    ];

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(palette.muted())
        .padding(Padding::proportional(1))
        .title(" COUNTER ")
        .title_alignment(Alignment::Center)
        .title_style(palette.accent().bold());

    let inner = block.inner(area);
    f.render_widget(block, area);
    center(f, inner, content);
}

fn render_palette(f: &mut Frame, area: Rect, app: &App, palette: Palette) {
    let entries: [(&str, Color); 6] = [
        ("muted", palette.muted),
        ("accent", palette.accent),
        ("success", palette.success),
        ("error", palette.error),
        ("warning", palette.warning),
        ("info", palette.info),
    ];

    let lines: Vec<Line> = entries
        .iter()
        .map(|(name, color)| {
            Line::from(vec![
                Span::styled("\u{25cf} ", Style::default().fg(*color)),
                Span::styled(format!(" {name:<8} "), Style::default().fg(*color)),
            ])
        })
        .collect();

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(palette.muted())
        .padding(Padding::proportional(1))
        .title(format!(" PALETTE · {} ", app.theme.name()))
        .title_alignment(Alignment::Center)
        .title_style(palette.accent().bold());

    let inner = block.inner(area);
    f.render_widget(block, area);
    center(f, inner, lines);
}

fn render_footer(f: &mut Frame, area: Rect, palette: Palette) {
    let help = Line::from(vec![
        Span::styled(" [t] ", palette.accent().bold()),
        Span::raw("switch theme"),
        Span::raw("   "),
        Span::styled(" [q] ", palette.accent().bold()),
        Span::raw("quit"),
    ]);
    f.render_widget(Paragraph::new(help), area);
}

fn center<'a>(f: &mut Frame, area: Rect, lines: Vec<Line<'a>>) {
    let centered = area.centered_vertically(Constraint::Length(lines.len() as u16));
    f.render_widget(Paragraph::new(lines).centered(), centered);
}
