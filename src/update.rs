use anyhow::Result;
use tracing::{debug, error, warn};

use crate::{
    action::Action,
    api::DNSQuery,
    app::{ActionState, ApiQueryResponseState, App, CurrentFocus, RunningState},
    port_check::{self, PortState},
};

impl App {
    pub fn update(&mut self, action: &Action) -> Result<()> {
        if *action != Action::Render {
            debug!("updating on new action: {action:?}");
        }
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
            Action::SetDNSStatus(dns_state) => {
                self.dns_status.query_response_state = Some(*dns_state);
            }
            Action::SetTCPPortState(port_state) => {
                self.dns_status.tcp_port_state = Some(*port_state);
            }
            Action::SetUDPPortState(port_state) => {
                self.dns_status.udp_port_state = Some(*port_state);
            }
            Action::UpdateTile => {
                if let CurrentFocus::DNSStatus = self.current_focus {
                    self.update_dns_tile();
                }
            }
            Action::RefreshLists => {
                self.refresh_blocking_lists();
            }
            Action::SetRefreshListState(action_state) => {
                self.blocking_list_refresh_state = Some(*action_state);
            }
            Action::ClearDNSCache => {
                self.clear_dns_cache();
            }
            Action::SetDNSCacheClearState(action_state) => {
                self.cache_delete_state = Some(*action_state);
            }
            _ => {}
        }
        Ok(())
    }

    fn clear_dns_cache(&self) {
        let tx = self.action_tx.clone();
        let api_client = self.api.clone();
        tokio::spawn(async move {
            tx.send(Action::SetDNSCacheClearState(ActionState::Waiting))
                .unwrap();
            match api_client.post_clear_dns_cache().await {
                Ok(resp) => {
                    if resp.status() == 200 {
                        debug!("successfully deleted DNS cache! {resp:?}");
                        tx.send(Action::SetDNSCacheClearState(ActionState::Success))
                            .unwrap()
                    } else {
                        warn!("deleting DNS cache did not work! {resp:?}");
                        tx.send(Action::SetDNSCacheClearState(ActionState::Failure))
                            .unwrap()
                    }
                }
                Err(err) => {
                    warn!("could not issue a DNS cache deletion POST command! {err}");
                    tx.send(Action::SetDNSCacheClearState(ActionState::Failure))
                        .unwrap()
                }
            }
        });
    }

    fn refresh_blocking_lists(&self) {
        let tx = self.action_tx.clone();
        let api_client = self.api.clone();
        tokio::spawn(async move {
            tx.send(Action::SetRefreshListState(ActionState::Waiting))
                .unwrap();
            match api_client.post_refresh_list_cmd().await {
                Ok(resp) => {
                    if resp.status() == 200 {
                        debug!("refreshing worked! {resp:?}");
                        tx.send(Action::SetRefreshListState(ActionState::Success))
                            .unwrap()
                    } else if resp.status() == 500 {
                        warn!("List refresh error {resp:?}");
                        tx.send(Action::SetRefreshListState(ActionState::Failure))
                            .unwrap()
                    } else {
                        warn!("received unknown response code from blocking list refresh command");
                        tx.send(Action::SetRefreshListState(ActionState::Failure))
                            .unwrap()
                    }
                }
                Err(err) => {
                    warn!("could not issue a refresh blocking lists POST command! {err}");
                    tx.send(Action::SetRefreshListState(ActionState::Failure))
                        .unwrap()
                }
            }
        });
    }

    fn update_dns_tile(&mut self) {
        let tx = self.action_tx.clone();
        let query = DNSQuery {
            query: "www.wikipedia.org",
            query_type: "A",
        };
        let dns_query = query.clone();
        let api_client = self.api.clone();
        let api_port = self.api.api_port;
        let dns_port = self.api.dns_port;
        tokio::spawn(async move {
            match api_client.post_dnsquery(dns_query).await {
                Ok(it) => {
                    if it.returnCode == "NOERROR" {
                        tx.send(Action::SetDNSStatus(ApiQueryResponseState::Healthy))
                            .unwrap()
                    } else {
                        tx.send(Action::SetDNSStatus(ApiQueryResponseState::Unhealthy))
                            .unwrap()
                    }
                }
                Err(err) => {
                    error!(%err);
                    tx.send(Action::SetDNSStatus(ApiQueryResponseState::NoResponse))
                        .unwrap()
                }
            };
        });

        let domain = self.api.url.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            match port_check::check_tcp_port(domain.to_string(), api_port).await {
                Ok(port_state) => {
                    tx.send(Action::SetTCPPortState(port_state)).unwrap();
                }
                Err(r) => {
                    error!("error testing TCP port: {:?}", r);
                    tx.send(Action::SetTCPPortState(PortState::Error)).unwrap();
                }
            }
        });

        let domain = self.api.url.clone();
        let dns_query = query.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            match port_check::check_dns(domain.to_string(), dns_port, dns_query).await {
                Ok(port_state) => {
                    tx.send(Action::SetUDPPortState(port_state)).unwrap();
                }
                Err(r) => {
                    error!("error querying UDP port: {:?}", r);
                    tx.send(Action::SetUDPPortState(PortState::Error)).unwrap();
                }
            }
        });
    }
}
