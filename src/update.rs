use std::time::Duration;

use anyhow::Result;
use tracing::debug;

use crate::{
    action::Action,
    app::{App, CurrentFocus, DNSStatus, RunningState},
};

impl App {
    pub fn update(&mut self, action: &Action) -> Result<()> {
        debug!("updating on new action: {action:?}");
        match action {
            Action::Quit => self.change_running_state(RunningState::Done),
            Action::JumpToTile(tile_num) => {
                self.set_tile_to_num(*tile_num);
                self.action_tx.send(Action::Render)?;
            }
            Action::CycleFocusUp => {
                self.cycle_focus_up();
                self.action_tx.send(Action::Render)?;
            }
            Action::CycleFocusDown => {
                self.cycle_focus_down();
                self.action_tx.send(Action::Render)?;
            }
            Action::SetDNSStatus(state) => {
                self.dns_status = Some(*state);
            }
            Action::UpdateTile => {
                if let CurrentFocus::DNSStatus = self.current_focus {
                    let tx = self.action_tx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_secs(3)).await;
                        tx.send(Action::SetDNSStatus(DNSStatus::Healthy)).unwrap();
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }
}
