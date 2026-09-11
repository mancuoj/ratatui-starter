use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Rect},
    macros::{horizontal, vertical},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

use crate::app::App;
use crate::theme::Palette;

const DIAMOND: &str = "◆";
const MIDDLE_DOT: &str = "·";

pub fn render(f: &mut Frame, app: &App) {
    let p = app.theme.palette();
    f.render_widget(Block::default().style(p.base()), f.area());

    let [_, main, footer, _] = vertical![== 1, >= 0, == 1, == 1].areas(f.area());
    let [counter, palette] = horizontal![== 40%, == 60%].areas(main);
    render_counter(f, counter, app, p);
    render_palette(f, palette, app, p);
    render_hint(
        f,
        footer,
        p,
        &[("t", "next theme"), ("T", "prev theme"), ("q", "quit")],
    );
}

fn render_counter(f: &mut Frame, area: Rect, app: &App, p: Palette) {
    let content = vec![
        Line::from(vec![
            Span::raw("Counter: "),
            Span::styled(format!("{:<4}", app.counter), p.accent().bold()),
        ]),
        Line::raw(""),
        Line::styled("j/k    change", p.muted()),
        Line::styled("r      reset", p.muted()),
    ];

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(p.border())
        .title(" COUNTER ")
        .title_alignment(Alignment::Center)
        .title_style(p.accent().bold());

    let inner = block.inner(area);
    f.render_widget(block, area);
    render_centered_lines(f, inner, content);
}

fn render_palette(f: &mut Frame, area: Rect, app: &App, p: Palette) {
    let swatches: [(&str, Style); 8] = [
        ("muted", p.muted()),
        ("border", p.border()),
        ("accent", p.accent()),
        ("success", p.success()),
        ("error", p.error()),
        ("warning", p.warning()),
        ("info", p.info()),
        ("selection", p.sel()),
    ];

    let lines: Vec<Line> = swatches
        .iter()
        .map(|(name, style)| Line::styled(format!(" {DIAMOND}  {name:<10}"), *style))
        .collect();

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(p.border())
        .title(format!(" PALETTE {MIDDLE_DOT} {} ", app.theme.name()))
        .title_alignment(Alignment::Center)
        .title_style(p.accent().bold());

    let inner = block.inner(area);
    f.render_widget(block, area);
    render_centered_lines(f, inner, lines);
}

fn render_hint<'a>(f: &mut Frame, area: Rect, p: Palette, entries: &[(&'a str, &'a str)]) {
    let mut spans = vec![Span::raw(" ")];
    for (index, (key, label)) in entries.iter().enumerate() {
        if index > 0 {
            spans.push(Span::styled(format!("  {MIDDLE_DOT}  "), p.muted()));
        }
        spans.push(Span::styled(*key, p.accent().bold()));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(*label, p.muted()));
    }
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_centered_lines<'a>(f: &mut Frame, area: Rect, lines: Vec<Line<'a>>) {
    let row = area.centered_vertically(Constraint::Length(lines.len() as u16));
    f.render_widget(Paragraph::new(lines).centered(), row);
}
