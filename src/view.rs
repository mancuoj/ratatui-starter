use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, List, ListItem, Padding, Paragraph},
};

use crate::{
    app::{App, Modal},
    theme,
};

pub fn draw(app: &mut App, frame: &mut Frame) {
    let [list_area, status_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .areas(frame.area());

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::border())
        .title(Line::from(vec![
            Span::styled(" Todos ", theme::key()),
            Span::styled(concat!("v", env!("CARGO_PKG_VERSION"), " "), theme::muted()),
        ]))
        .padding(Padding::uniform(1));

    if app.todos.is_empty() {
        let inner = block.inner(list_area);
        frame.render_widget(block, list_area);
        frame.render_widget(
            Paragraph::new("what needs to be done?")
                .style(theme::muted())
                .alignment(Alignment::Center),
            Rect {
                y: inner.y + inner.height.saturating_sub(1) / 2,
                height: inner.height.min(1),
                ..inner
            },
        );
    } else {
        let selected = app.list_state.selected();
        let items = app.todos.iter().enumerate().map(|(index, todo)| {
            let marker = if selected == Some(index) {
                Span::styled("▌ ", theme::accent())
            } else {
                Span::raw("  ")
            };
            let check = if todo.done {
                Span::styled("● ", theme::success())
            } else {
                Span::styled("○ ", theme::muted())
            };
            let title = if todo.done {
                theme::completed()
            } else if selected == Some(index) {
                theme::text()
            } else {
                theme::secondary()
            };
            ListItem::new(Line::from(vec![
                marker,
                check,
                Span::styled(&todo.title, title),
            ]))
        });
        let list = List::new(items)
            .block(block)
            .highlight_style(theme::selection());
        frame.render_stateful_widget(list, list_area, &mut app.list_state);
    }

    let hints = match app.modal {
        Modal::None => [
            ("j/k", "move"),
            ("space", "toggle"),
            ("a", "add"),
            ("d", "delete"),
            ("q", "quit"),
        ]
        .as_slice(),
        Modal::Add => [("⏎", "save"), ("esc", "cancel")].as_slice(),
        Modal::ConfirmDelete { .. } => [("y", "yes"), ("n", "no")].as_slice(),
    };
    frame.render_widget(Paragraph::new(status_line(hints)), status_area);

    match app.modal {
        Modal::Add => draw_add_modal(app, frame),
        Modal::ConfirmDelete { index } => draw_delete_modal(app, frame, index),
        Modal::None => {}
    }
}

fn draw_add_modal(app: &App, frame: &mut Frame) {
    let area = centered(frame.area(), 52, 5);
    frame.render_widget(Clear, area);
    let block = modal_block("Add todo");
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(app.input.as_str(), theme::text()),
            Span::styled("▌", theme::accent()),
        ])),
        Rect { height: 1, ..inner },
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("⏎ ", theme::key()),
            Span::styled("save", theme::muted()),
            Span::styled("   esc ", theme::key()),
            Span::styled("cancel", theme::muted()),
        ])),
        Rect {
            y: inner.y + 2,
            height: 1,
            ..inner
        },
    );
}

fn draw_delete_modal(app: &App, frame: &mut Frame, index: usize) {
    let title = app
        .todos
        .get(index)
        .map(|todo| todo.title.as_str())
        .unwrap_or("this todo");
    let message = format!("Delete \"{title}\"?");
    let width = (message.chars().count() as u16 + 6).clamp(30, 70);
    let area = centered(frame.area(), width, 5);
    frame.render_widget(Clear, area);
    let block = modal_block("Confirm");
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(
        Paragraph::new(Line::styled(message, theme::text())),
        Rect { height: 1, ..inner },
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("y ", theme::key()),
            Span::styled("delete", theme::danger()),
            Span::styled("   n ", theme::key()),
            Span::styled("keep it", theme::muted()),
        ])),
        Rect {
            y: inner.y + 2,
            height: 1,
            ..inner
        },
    );
}

fn modal_block(title: &str) -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::accent())
        .title(Span::styled(format!(" {title} "), theme::key()))
        .padding(Padding::horizontal(1))
}

fn status_line(hints: &[(&str, &str)]) -> Line<'static> {
    let mut spans = vec![Span::raw(" ")];
    for (index, (key, description)) in hints.iter().enumerate() {
        if index > 0 {
            spans.push(Span::styled("  ·  ", theme::border()));
        }
        spans.push(Span::styled((*key).to_string(), theme::key()));
        spans.push(Span::raw(" "));
        spans.push(Span::styled((*description).to_string(), theme::muted()));
    }
    Line::from(spans)
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width.saturating_sub(2));
    let height = height.min(area.height.saturating_sub(2));
    Rect {
        x: area.x + (area.width - width) / 2,
        y: area.y + (area.height - height) / 2,
        width,
        height,
    }
}
