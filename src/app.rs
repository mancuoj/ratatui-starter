use ratatui::widgets::ListState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Modal {
    #[default]
    None,
    Add,
    ConfirmDelete {
        index: usize,
    },
}

#[derive(Debug)]
pub struct App {
    pub todos: Vec<Todo>,
    pub list_state: ListState,
    pub input: String,
    pub modal: Modal,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            todos: vec![
                Todo {
                    title: "Explore the starter".into(),
                    done: true,
                },
                Todo {
                    title: "Build something useful".into(),
                    done: false,
                },
            ],
            list_state,
            input: String::new(),
            modal: Modal::None,
            should_quit: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    MoveUp,
    MoveDown,
    Toggle,
    OpenAdd,
    OpenDelete,
    Input(char),
    Backspace,
    SubmitAdd,
    ConfirmDelete,
    CancelModal,
    Quit,
}

pub fn update(app: &mut App, message: Message) -> Option<Message> {
    match message {
        Message::MoveUp => move_selection(app, -1),
        Message::MoveDown => move_selection(app, 1),
        Message::Toggle => {
            if let Some(todo) = selected_todo_mut(app) {
                todo.done = !todo.done;
            }
        }
        Message::OpenAdd => {
            app.input.clear();
            app.modal = Modal::Add;
        }
        Message::OpenDelete => {
            if let Some(index) = app.list_state.selected().filter(|&i| i < app.todos.len()) {
                app.modal = Modal::ConfirmDelete { index };
            }
        }
        Message::Input(c) => app.input.push(c),
        Message::Backspace => {
            app.input.pop();
        }
        Message::SubmitAdd => {
            if let Modal::Add = app.modal {
                let title = app.input.trim().to_string();
                if !title.is_empty() {
                    app.todos.push(Todo { title, done: false });
                    app.list_state.select(Some(app.todos.len() - 1));
                }
                close_modal(app);
            }
        }
        Message::ConfirmDelete => {
            if let Modal::ConfirmDelete { index } = app.modal {
                if index < app.todos.len() {
                    app.todos.remove(index);
                    app.list_state.select(if app.todos.is_empty() {
                        None
                    } else {
                        Some(index.min(app.todos.len() - 1))
                    });
                }
                close_modal(app);
            }
        }
        Message::CancelModal => close_modal(app),
        Message::Quit => app.should_quit = true,
    }
    None
}

fn close_modal(app: &mut App) {
    app.input.clear();
    app.modal = Modal::None;
}

fn move_selection(app: &mut App, direction: i32) {
    if app.todos.is_empty() {
        return;
    }
    let current = app.list_state.selected().unwrap_or(0) as i32;
    let next = (current + direction).rem_euclid(app.todos.len() as i32) as usize;
    app.list_state.select(Some(next));
}

fn selected_todo_mut(app: &mut App) -> Option<&mut Todo> {
    app.list_state.selected().and_then(|i| app.todos.get_mut(i))
}
