use crate::{
    app::{App, RunningState},
    event::Message,
};

pub fn update(app: &mut App, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => app.change_running_state(RunningState::Done),
        Message::CycleFocusUp => app.cycle_focus_up(),
        Message::CycleFocusDown => app.cycle_focus_down(),
        _ => {}
    }
    None
}
