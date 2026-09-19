use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event;
use starter::{app::App, input, model::Msg, ui};

const TICK_RATE: Duration = Duration::from_millis(100);

fn main() -> Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| run(&mut app, terminal))
}

fn run(app: &mut App, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)?
            && let Some(key) = event::read()?.as_key_press_event()
            && let Some(msg) = input::translate(app, key)
        {
            app.update(msg);
        }

        if last_tick.elapsed() >= TICK_RATE {
            app.update(Msg::Tick);
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
