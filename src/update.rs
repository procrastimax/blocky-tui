use crossterm::event::KeyCode;

use crate::{
    app::{App, RunningState},
    event::Message,
};

pub fn update(app: &mut App, msg: Message) {
    match msg {
        Message::Key(e) => match e.code {
            KeyCode::Esc => app.change_running_state(RunningState::Done),
            KeyCode::Char('q') => app.change_running_state(RunningState::Done),
            _ => {}
        },
        Message::Quit => app.change_running_state(RunningState::Done),
        _ => {}
    }
}
