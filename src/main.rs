use anyhow::Result;
use crossterm::event;
use starter::{app::App, input, ui};

fn main() -> Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| run(&mut app, terminal))
}

fn run(app: &mut App, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if let Some(key) = event::read()?.as_key_press_event()
            && let Some(msg) = input::translate(app, key)
        {
            app.update(msg);
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
