use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Rect},
    macros::{horizontal, line, vertical},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

use crate::app::{App, Focus, Overlay};
use crate::theme::Palette;

pub mod modal;

const DIAMOND: &str = "◆";
const CIRCLE: &str = "●";
const MIDDLE_DOT: &str = "·";
const SPINNER: [&str; 10] = ["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"];

pub fn render(f: &mut Frame, app: &App) {
    let p = app.theme.palette();
    f.render_widget(Block::default().style(p.base()), f.area());

    let [_, main, footer, _] = vertical![== 1, >= 0, == 1, == 1].areas(f.area());
    let [counter, palette] = horizontal![== 40%, == 60%].areas(main);
    render_counter(f, counter, app, p);
    render_palette(f, palette, app, p);
    render_hint(f, footer, app, p);

    modal::render(f, app, p);
}

fn render_counter(f: &mut Frame, area: Rect, app: &App, p: Palette) {
    let focus = app.focus == Focus::Counter;

    let spinner = SPINNER[(app.tick as usize) % SPINNER.len()];
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(p.border(focus))
        .title(Line::styled(format!(" {spinner} COUNTER "), p.title(focus)))
        .title_alignment(Alignment::Center);

    let content = vec![line![
        "Counter: ",
        format!("{:<4}", app.counter).fg(p.accent).bold()
    ]];

    let inner = block.inner(area);
    f.render_widget(block, area);
    centered_lines(f, inner, content);
}

fn render_palette(f: &mut Frame, area: Rect, app: &App, p: Palette) {
    let focus = app.focus == Focus::Palette;

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(p.border(focus))
        .title(Line::styled(
            format!(" PALETTE {MIDDLE_DOT} {} ", app.theme.name()),
            p.title(focus),
        ))
        .title_alignment(Alignment::Center);

    let swatches: [(&str, Style); 8] = [
        ("muted", p.muted.into()),
        ("border", p.border.into()),
        ("accent", p.accent.into()),
        ("success", p.success.into()),
        ("error", p.error.into()),
        ("warning", p.warning.into()),
        ("info", p.info.into()),
        ("selection", p.sel(true)),
    ];

    let lines: Vec<Line> = swatches
        .iter()
        .map(|(name, style)| Line::styled(format!(" {DIAMOND}  {name:<10}"), *style))
        .collect();

    let inner = block.inner(area);
    f.render_widget(block, area);
    centered_lines(f, inner, lines);
}

fn render_hint(f: &mut Frame, area: Rect, app: &App, p: Palette) {
    let entries: &[(&str, &str)] = match app.overlay {
        Overlay::Theme { .. } => &[("j/k", "preview"), ("enter", "apply"), ("esc", "cancel")],
        Overlay::None => match app.focus {
            Focus::Counter => &[
                ("q", "quit"),
                ("tab", "tabs"),
                ("j/k", "change"),
                ("r", "reset"),
            ],
            Focus::Palette => &[("q", "quit"), ("tab", "tabs"), ("t", "themes")],
        },
    };

    let mut spans = vec![Span::raw(" ")];
    for (index, (key, label)) in entries.iter().enumerate() {
        if index > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push((*key).fg(p.accent).bold());
        spans.push(Span::raw(" "));
        spans.push((*label).fg(p.muted));
    }
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn centered_lines<'a>(f: &mut Frame, area: Rect, lines: Vec<Line<'a>>) {
    let row = area.centered_vertically(Constraint::Length(lines.len() as u16));
    f.render_widget(Paragraph::new(lines).centered(), row);
}
