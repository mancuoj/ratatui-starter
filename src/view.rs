use ratatui::{
    Frame,
    widgets::{Block, BorderType, Paragraph},
};

use crate::model::Model;

pub fn view(model: &mut Model, frame: &mut Frame) {
    let block = Block::bordered().border_type(BorderType::Rounded);
    let p = Paragraph::new(format!("Counter: {}", model.counter))
        .centered()
        .block(block);
    frame.render_widget(p, frame.area());
}
