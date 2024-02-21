use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, RunningState},
    event::Message,
};

pub fn update(app: &mut App, msg: Message) -> Option<Message> {
    match msg {
        Message::Key(e) => return Some(Message::Quit),
        Message::Quit => app.change_running_state(RunningState::Done),
        _ => {}
    }
    None
}
