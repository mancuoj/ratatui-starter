#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Msg {
    Quit,
    Increment,
    Decrement,
    Reset,
}

pub struct App {
    pub should_quit: bool,
    pub counter: i64,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            counter: 0,
        }
    }

    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => self.should_quit = true,
            Msg::Increment => self.counter += 1,
            Msg::Decrement => self.counter -= 1,
            Msg::Reset => self.counter = 0,
        }
    }
}
