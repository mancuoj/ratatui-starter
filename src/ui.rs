use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

use crate::app::App;
use crate::theme::Palette;

pub fn render(f: &mut Frame, app: &App) {
    let p = app.theme.palette();

    f.render_widget(Block::default().style(p.base()), f.area());

    let [_, main, footer, _] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(f.area());

    let [counter, pallette] =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]).areas(main);
    render_counter(f, counter, app, p);
    render_pallette(f, pallette, app, p);

    render_footer(f, footer, p);
}

fn render_counter(f: &mut Frame, area: Rect, app: &App, p: Palette) {
    let content = vec![
        Line::from(vec![
            Span::raw("Counter: "),
            Span::styled(format!("{}", app.counter), p.accent().bold()),
        ]),
        Line::raw(""),
        Line::raw("j/k    change"),
        Line::raw("r      reset"),
    ];

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(p.muted())
        .title(" COUNTER ")
        .title_alignment(Alignment::Center)
        .title_style(p.accent().bold());

    let inner = block.inner(area);
    f.render_widget(block, area);
    render_centered_lines(f, inner, content);
}

fn render_pallette(f: &mut Frame, area: Rect, app: &App, p: Palette) {
    let entries: [(&str, Color); 6] = [
        ("muted", p.muted),
        ("accent", p.accent),
        ("success", p.success),
        ("error", p.error),
        ("warning", p.warning),
        ("info", p.info),
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
        .border_style(p.muted())
        .title(format!(" PALETTE · {} ", app.theme.name()))
        .title_alignment(Alignment::Center)
        .title_style(p.accent().bold());

    let inner = block.inner(area);
    f.render_widget(block, area);
    render_centered_lines(f, inner, lines);
}

fn render_footer(f: &mut Frame, area: Rect, p: Palette) {
    let help = Line::from(vec![
        Span::styled(" [t] ", p.accent().bold()),
        Span::raw("switch theme"),
        Span::raw("   "),
        Span::styled(" [q] ", p.accent().bold()),
        Span::raw("quit"),
    ]);
    f.render_widget(Paragraph::new(help), area);
}

fn render_centered_lines<'a>(f: &mut Frame, area: Rect, lines: Vec<Line<'a>>) {
    let centered = area.centered_vertically(Constraint::Length(lines.len() as u16));
    f.render_widget(Paragraph::new(lines).centered(), centered);
}
