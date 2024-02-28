use anyhow::Result;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tracing::{debug, info};

use crate::action::Action;
use crate::api::ApiClient;
use crate::port_check::PortState;
use crate::tui::{self};

#[derive(Debug)]
pub struct App {
    pub api: ApiClient,
    pub action_tx: UnboundedSender<Action>,
    pub action_rx: UnboundedReceiver<Action>,
    // tui: Tui,
    pub running_state: RunningState,
    pub current_screen: CurrentScreen,
    pub current_focus: CurrentFocus,
    /// tracking whether the user is currently inputting something in a text field
    pub is_currently_editing: bool,
    pub blocking_status: Option<BlockingStatus>,
    pub dns_status: DNSStatus,
    pub cache_delete_status: Option<CacheDeleteStatus>,
}

#[derive(Debug)]
pub enum CacheDeleteStatus {
    Success,
    Failure,
}

/// Represents the state of the blocky DNS server
///
/// Keeps track of the TCP port state, UDP port state and the result of an API DNS Query
#[derive(Debug)]
pub struct DNSStatus {
    pub query_response_state: Option<ApiQueryResponseState>,
    pub tcp_port_state: Option<PortState>,
    pub udp_port_state: Option<PortState>,
}

impl Default for DNSStatus {
    fn default() -> Self {
        Self {
            query_response_state: None,
            tcp_port_state: None,
            udp_port_state: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ApiQueryResponseState {
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

impl App {
    pub fn new() -> Result<Self> {
        let api = ApiClient::new("http://localhost", 4000, 1234)?;
        let (action_tx, action_rx) = unbounded_channel::<Action>();
        let app = Self {
            api,
            action_tx: action_tx.clone(),
            action_rx,
            running_state: RunningState::Running,
            current_screen: CurrentScreen::Main,
            current_focus: CurrentFocus::DNSStatus,
            is_currently_editing: false,
            blocking_status: None,
            dns_status: DNSStatus::default(),
            cache_delete_status: None,
        };
        debug!("created new app struct");
        Ok(app)
    }
    pub async fn run(&mut self) -> Result<()> {
        let mut tui = tui::Tui::new()?.frame_rate(3.0);
        tui.enter()?;
        info!("starting main app loop");
        loop {
            if let Some(evt) = tui.next().await {
                self.handle_event(&evt)?;

                while let Ok(action) = self.action_rx.try_recv() {
                    self.update(&action)?;
                    if let Action::Render = action {
                        tui.draw(|f| {
                            self.render(f);
                        })?;
                    }
                }
            };
            if self.running_state == RunningState::Done {
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    pub fn change_running_state(&mut self, state: RunningState) {
        self.running_state = state
    }

    pub fn cycle_focus_up(&mut self) {
        self.current_focus.increase();
    }
    pub fn cycle_focus_down(&mut self) {
        self.current_focus.decrease();
    }
    pub fn set_tile_to_num(&mut self, num: u8) {
        self.current_focus.set_on_number(num);
    }
}
