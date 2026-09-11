use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Rect},
    macros::{horizontal, line, vertical},
    style::{Style, Stylize},
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
        line![
            "Counter: ",
            format!("{:<4}", app.counter).fg(p.accent).bold()
        ],
        Line::default(),
        Line::from("j/k    change".fg(p.muted)),
        Line::from("r      reset".fg(p.muted)),
    ];

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(p.border)
        .title(" COUNTER ".fg(p.accent).bold())
        .title_alignment(Alignment::Center);

    let inner = block.inner(area);
    f.render_widget(block, area);
    render_centered_lines(f, inner, content);
}

fn render_palette(f: &mut Frame, area: Rect, app: &App, p: Palette) {
    let swatches: [(&str, Style); 8] = [
        ("muted", p.muted.into()),
        ("border", p.border.into()),
        ("accent", p.accent.into()),
        ("success", p.success.into()),
        ("error", p.error.into()),
        ("warning", p.warning.into()),
        ("info", p.info.into()),
        ("selection", p.sel()),
    ];

    let lines: Vec<Line> = swatches
        .iter()
        .map(|(name, style)| Line::styled(format!(" {DIAMOND}  {name:<10}"), *style))
        .collect();

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(p.border)
        .title(
            format!(" PALETTE {MIDDLE_DOT} {} ", app.theme.name())
                .fg(p.accent)
                .bold(),
        )
        .title_alignment(Alignment::Center);

    let inner = block.inner(area);
    f.render_widget(block, area);
    render_centered_lines(f, inner, lines);
}

fn render_hint<'a>(f: &mut Frame, area: Rect, p: Palette, entries: &[(&'a str, &'a str)]) {
    let mut spans = vec![Span::raw(" ")];
    for (index, (key, label)) in entries.iter().enumerate() {
        if index > 0 {
            spans.push(format!("  {MIDDLE_DOT}  ").fg(p.muted));
        }
        spans.push((*key).fg(p.accent).bold());
        spans.push(Span::raw(" "));
        spans.push((*label).fg(p.muted));
    }
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_centered_lines<'a>(f: &mut Frame, area: Rect, lines: Vec<Line<'a>>) {
    let row = area.centered_vertically(Constraint::Length(lines.len() as u16));
    f.render_widget(Paragraph::new(lines).centered(), row);
}
