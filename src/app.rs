// TODO: implement refresh list refresh feature
// TODO: implement disable blocking for specific groups

use anyhow::{bail, Result};

#[derive(Debug)]
pub struct App {
    pub running_state: RunningState,
    pub current_screen: CurrentScreen,
    pub current_focus: CurrentFocus,
    pub blocking_status: BlockingStatus,
    pub dns_status: Option<DNSStatus>,
}

#[derive(Debug, Default)]
struct DNSResponse {
    status: u32,
    message: String,
}

/// Represents the state of the blocky DNS server
///
/// Healthy -> DNS is properly working
/// Unreachable -> DNS is not reachable, probably wrong IP/ Hostname/ Port
/// NoResponse -> DNS is reachable, but does not respond with DNS responses
#[derive(Debug)]
enum DNSStatus {
    Healthy,
    Unreachable,
    NoResponse,
}

/// Represents the blocking status of blocky
///
/// Enabled -> Blocking is enabled
/// Disabled -> Blocking is disabled
#[derive(Debug, PartialEq, Eq)]
pub struct BlockingStatus {
    is_blocking_enabled: bool,
    // Number of seconds the blocking is disabled
    // if None than blocking is disabled permanently,
    // if 0 than blocking is enabled
    unblocking_timer: Option<i32>,
}

impl Default for BlockingStatus {
    fn default() -> Self {
        Self {
            is_blocking_enabled: true,
            unblocking_timer: Some(0),
        }
    }
}

/// Store the currently focused tile.
///
/// Disable -> Tile to handle disabling DNS blocking
/// Lists -> Tile to handle refreshing of blacklists and whitelists
/// Status -> Tile to show current status of blocky
/// Query -> Tile handling custom DNS query
#[derive(Debug, Default, PartialEq, Eq)]
pub enum CurrentFocus {
    #[default]
    Status,
    Lists,
    Disable,
    Query,
}

/// Stores the currently shown screen.
///
/// Setup -> Initial Setup Dialog (TODO)
/// Main -> Overview of all Tiles
/// Exiting -> Confirm Exit (TODO)
#[derive(Debug, Default, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Setup,
    Exiting,
}

/// Stores the current app's running state.
///
/// Running -> App is properly running
/// Done -> App shoudl get closed
#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

impl App {
    pub fn new() -> Self {
        Self {
            running_state: RunningState::Running,
            current_screen: CurrentScreen::Main,
            current_focus: CurrentFocus::Status,
            blocking_status: BlockingStatus::default(),
            dns_status: None,
        }
    }

    pub fn change_running_state(&mut self, state: RunningState) {
        self.running_state = state
    }

    pub fn get_dns_state(&mut self) {}

    pub fn get_blocking_state(&mut self) -> Result<BlockingStatus> {
        bail!(r#"Placeholder"#)
    }

    pub fn set_blocking_state(&mut self, status: BlockingStatus) {}

    pub fn query_dns_server(&mut self, query_request: String) {}

    pub fn refresh_blocking_lists(&mut self) {}
}
