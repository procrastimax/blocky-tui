use std::str::FromStr;
use std::time::Duration;

use anyhow::{anyhow, Result};
use rustdns::{Message, Type};
use tokio::net::{TcpStream, UdpSocket};
use tokio::time::timeout;
use tracing::debug;
use url::Url;

use crate::api::DNSQuery;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PortState {
    Open,
    Closed,
    Error,
}

pub async fn check_tcp_port(host: String, tcp_port: u16) -> Result<PortState> {
    debug!("checking API server TCP port by manually creating a connection");
    if let Some(domain) = Url::parse(&host)?.host_str() {
        match TcpStream::connect(format!("{domain}:{tcp_port}")).await {
            Ok(_) => Ok(PortState::Open),
            Err(r) => {
                debug!("could not create TCPStream: {r}");
                Ok(PortState::Closed)
            }
        }
    } else {
        Err(anyhow!("could not get host from host string"))
    }
}

pub async fn check_dns(host: String, udp_port: u16, query: DNSQuery) -> Result<PortState> {
    debug!("checking DNS by manually quering it");
    if let Some(domain) = Url::parse(&host)?.host_str() {
        let sock = UdpSocket::bind("localhost:9641").await?;
        sock.connect(format!("{domain}:{udp_port}")).await?;

        let mut m = Message::default();
        // NOTE: this could cause more troubles than manually parsing it
        let query_type = Type::from_str(query.query_type)?;
        m.add_question(domain, query_type, rustdns::Class::Internet);

        let question = m.to_vec()?;
        sock.send(&question).await?;

        let mut resp = [0; 4096];
        // NOTE: the udp port is seen as closed only when the response times out
        let len = match timeout(Duration::from_secs(5), sock.recv(&mut resp)).await {
            Ok(resp_res) => resp_res?,
            Err(timeout) => {
                debug!("UDP DNS request timed out: {timeout}");
                return Ok(PortState::Closed);
            }
        };

        let answer = Message::from_slice(&resp[0..len])?;
        debug!("received dns response: {answer}");

        Ok(PortState::Open)
    } else {
        Err(anyhow!("could not get host from host string"))
    }
}
