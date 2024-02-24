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
    pub cache_delete_status: Option<CacheDeleteStatus>,
}

#[derive(Debug)]
enum CacheDeleteStatus {
    Success,
    Failure,
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
#[derive(Debug, PartialEq, Eq)]
pub struct BlockingStatus {
    /// true if blocking is enabled
    pub is_blocking_enabled: bool,
    ///  If blocking is temporary disabled: amount of seconds until blocking will be enabled
    pub unblocking_timer: Option<u32>,
    /// Disabled group names
    disabled_groups: Option<String>,
}

/// Store the currently focused tile.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum CurrentFocus {
    #[default]
    DNSStatus = 1,
    BlockingStatus,
    RefreshLists,
    DeleteCache,
    QueryDNS,
}

impl CurrentFocus {
    fn increase(&mut self) {
        *self = match self {
            CurrentFocus::DNSStatus => CurrentFocus::BlockingStatus,
            CurrentFocus::BlockingStatus => CurrentFocus::RefreshLists,
            CurrentFocus::RefreshLists => CurrentFocus::DeleteCache,
            CurrentFocus::DeleteCache => CurrentFocus::QueryDNS,
            CurrentFocus::QueryDNS => CurrentFocus::DNSStatus,
        }
    }
    fn decrease(&mut self) {
        *self = match self {
            CurrentFocus::QueryDNS => CurrentFocus::DeleteCache,
            CurrentFocus::DeleteCache => CurrentFocus::RefreshLists,
            CurrentFocus::RefreshLists => CurrentFocus::BlockingStatus,
            CurrentFocus::BlockingStatus => CurrentFocus::DNSStatus,
            CurrentFocus::DNSStatus => CurrentFocus::QueryDNS,
        }
    }
    fn set_on_number(&mut self, number: u8) {
        *self = match number {
            1 => CurrentFocus::DNSStatus,
            2 => CurrentFocus::BlockingStatus,
            3 => CurrentFocus::RefreshLists,
            4 => CurrentFocus::DeleteCache,
            5 => CurrentFocus::QueryDNS,
            _ => CurrentFocus::DNSStatus,
        }
    }
    pub fn get_tile_number(&self) -> u8 {
        match self {
            CurrentFocus::DNSStatus => 1,
            CurrentFocus::BlockingStatus => 2,
            CurrentFocus::RefreshLists => 3,
            CurrentFocus::DeleteCache => 4,
            CurrentFocus::QueryDNS => 5,
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
            cache_delete_status: None,
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
