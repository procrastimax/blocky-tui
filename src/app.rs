// TODO: implement refresh list refresh feature
// TODO: implement disable blocking for specific groups

use anyhow::{bail, Result};

#[derive(Debug)]
pub struct App {
    pub running_state: RunningState,
    pub current_screen: CurrentScreen,
    pub current_focus: CurrentFocus,
    /// tracking whether the user is currently inputting something in a text field
    pub is_currently_editing: bool,
    pub blocking_status: Option<BlockingStatus>,
    pub dns_status: Option<DNSStatus>,
}

#[derive(Debug, Default)]
struct DNSResponse {
    status: u32,
    message: String,
}

/// Represents the state of the blocky DNS server
#[derive(Debug)]
pub enum DNSStatus {
    /// Healthy -> DNS is properly working
    Healthy,
    /// Unhealthy -> received wrong or not working DNS response
    Unhealthy,
    /// NoResponse -> DNS is reachable, but does not respond with DNS responses
    NoResponse,
}

/// Represents the blocking status of blocky
///
/// Enabled -> Blocking is enabled
/// Disabled -> Blocking is disabled
#[derive(Debug, PartialEq, Eq)]
pub struct BlockingStatus {
    pub is_blocking_enabled: bool,
    // Number of seconds the blocking is disabled
    // if None than blocking is disabled permanently,
    // if 0 than blocking is enabled
    pub unblocking_timer: Option<i32>,
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
    DNSStatus,
    BlockingStatus,
    RefreshLists,
    QueryDNS,
}

impl CurrentFocus {
    fn increase(&mut self) {
        *self = match self {
            CurrentFocus::DNSStatus => CurrentFocus::BlockingStatus,
            CurrentFocus::BlockingStatus => CurrentFocus::RefreshLists,
            CurrentFocus::RefreshLists => CurrentFocus::QueryDNS,
            CurrentFocus::QueryDNS => CurrentFocus::DNSStatus,
        }
    }
    fn decrease(&mut self) {
        *self = match self {
            CurrentFocus::QueryDNS => CurrentFocus::RefreshLists,
            CurrentFocus::RefreshLists => CurrentFocus::BlockingStatus,
            CurrentFocus::BlockingStatus => CurrentFocus::DNSStatus,
            CurrentFocus::DNSStatus => CurrentFocus::QueryDNS,
        }
    }
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

impl Default for App {
    fn default() -> Self {
        Self {
            running_state: RunningState::Running,
            current_screen: CurrentScreen::Main,
            current_focus: CurrentFocus::DNSStatus,
            is_currently_editing: false,
            blocking_status: None,
            dns_status: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
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

    pub fn cycle_focus_up(&mut self) {
        self.current_focus.increase();
    }
    pub fn cycle_focus_down(&mut self) {
        self.current_focus.decrease();
    }
}
