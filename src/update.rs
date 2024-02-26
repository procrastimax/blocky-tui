use crate::{
    api::{BlockyApi, DNSQuery},
    app::{App, CurrentFocus, DNSStatus, RunningState},
    event::Message,
};

pub fn update(app: &mut App, api: &BlockyApi, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => app.change_running_state(RunningState::Done),
        Message::JumpToTile(tile_num) => app.set_tile_to_num(tile_num),
        Message::CycleFocusUp => app.cycle_focus_up(),
        Message::CycleFocusDown => app.cycle_focus_down(),
        Message::UpdateTile => {
            if let CurrentFocus::DNSStatus = app.current_focus {
                let dns_response = api.post_dnsquery(DNSQuery {
                    query: "google.com".to_string(),
                    query_type: "A".to_string(),
                });
                match dns_response {
                    Ok(response) => {
                        app.dns_status = Some(DNSStatus::Healthy);
                    }
                    Err(e) => {
                        app.dns_status = Some(DNSStatus::NoResponse);
                    }
                }
            }
        }
        _ => {}
    }
    None
}
