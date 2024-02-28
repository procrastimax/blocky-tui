use anyhow::Result;
use tracing::{debug, error};

use crate::{
    action::Action,
    api::DNSQuery,
    app::{ApiQueryResponseState, App, CurrentFocus, RunningState},
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
            _ => {}
        }
        Ok(())
    }
}
