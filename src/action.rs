use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tracing::debug;

use crate::app::{App, DNSStatus};
use crate::tui::Event;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Init,
    CycleFocusUp,   // Move focus to next tile in UI
    CycleFocusDown, // Move focus to previous tile in UI
    JumpToTile(u8),
    EnableDNSBlocking,  // enables DNS blocking
    DisableDNSBlocking, // disable DNS blocking
    SubmitDNSQuery,     // sends DNS query to blocky
    RefreshLists,       // Refresh blocking lists
    UpdateTile,         // Update current Tile (or all app information)
    Key(KeyEvent),
    SetDNSStatus(DNSStatus),
    Render,
    Quit, // quits application
}

impl App {
    pub fn handle_event(&self, event: &Event) -> Result<()> {
        debug!("handling new event: {event:?}");
        match event {
            Event::Init => self.action_tx.send(Action::Init)?,
            Event::Key(key) => self.handle_key(key)?,
            Event::Quit => self.action_tx.send(Action::Quit)?,
            Event::Render => self.action_tx.send(Action::Render)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_key(&self, key: &KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.action_tx.send(Action::Quit)?,
            KeyCode::Char('q') => {
                if !self.is_currently_editing {
                    self.action_tx.send(Action::Quit)?
                } else {
                    self.action_tx.send(Action::Key(*key))?
                }
            }
            KeyCode::Char('c') => {
                if key.modifiers == KeyModifiers::CONTROL {
                    self.action_tx.send(Action::Quit)?
                } else {
                    self.action_tx.send(Action::Key(*key))?
                }
            }
            KeyCode::Char('R') => {
                if !self.is_currently_editing {
                    self.action_tx.send(Action::UpdateTile)?
                } else {
                    self.action_tx.send(Action::Key(*key))?
                }
            }
            KeyCode::Char(val) => {
                if !self.is_currently_editing && val.is_numeric() {
                    // subtract 48 as u8, since the char->u8 conversion converts to ascii code
                    // so char('1') is 49 in u8
                    self.action_tx
                        .send(Action::JumpToTile((val as u8) - 48u8))?
                } else {
                    self.action_tx.send(Action::Key(*key))?
                }
            }
            KeyCode::Tab => self.action_tx.send(Action::CycleFocusUp)?,
            KeyCode::BackTab => self.action_tx.send(Action::CycleFocusDown)?,
            _ => {}
        }
        Ok(())
    }
}
