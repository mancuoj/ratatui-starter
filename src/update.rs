use crate::{model::Model, msg::Msg};

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Quit => model.should_quit = true,
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
        Msg::Reset => model.counter = 0,
    }
}
