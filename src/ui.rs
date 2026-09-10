use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::Stylize,
    widgets::{Block, BorderType, Padding, Paragraph},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let [_, main, footer, _] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    let content = format!("Counter: {}", app.counter);
    frame.render_widget(
        Paragraph::new(content).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .padding(Padding::proportional(1))
                .title(" STARTER ")
                .title_alignment(Alignment::Center)
                .bold(),
        ),
        main,
    );
    frame.render_widget(Paragraph::new(" [r] reset   [q] quit"), footer);
}
