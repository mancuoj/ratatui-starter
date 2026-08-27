use anyhow::Result;
use ratatui::DefaultTerminal;

use ratatui_starter::app::App;
use ratatui_starter::{app, event, view};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let result = run(&mut terminal, &mut app);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| view::draw(app, frame))?;

        let mut message = event::handle_event(app)?;
        while let Some(msg) = message {
            message = app::update(app, msg);
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
