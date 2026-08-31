use anyhow::Result;
use ratatui::DefaultTerminal;

use crate::model::Model;

mod event;
mod model;
mod msg;
mod update;
mod view;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut model = Model::default();
    let result = run(&mut terminal, &mut model);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal, model: &mut Model) -> Result<()> {
    while !model.should_quit {
        terminal.draw(|frame| view::view(model, frame))?;

        if let Some(msg) = event::handle_event()? {
            update::update(model, msg);
        }
    }
    Ok(())
}
