use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use std::time::Duration;
use tracing::{debug, error};
use url::Url;

#[derive(Debug, Clone)]
pub struct ApiClient {
    /// Blocky API Base Url
    pub url: Url,
    pub dns_port: u16,
    pub api_port: u16,
    client: reqwest::Client,
}

#[derive(Debug, Serialize, Clone)]
pub struct DNSQuery {
    pub query: &'static str,
    #[serde(rename = "type")]
    pub query_type: &'static str,
}

#[allow(non_snake_case)]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DNSResponse {
    pub reason: String,
    pub response: String,
    pub responseType: String,
    pub returnCode: String,
}

impl ApiClient {
    pub fn new(base_url: &'static str, api_port: u16, dns_port: u16) -> Result<Self> {
        let mut url = Url::parse(base_url)?;
        match url.scheme() {
            "http" | "https" => {}
            "" => {
                if url.set_scheme("https").is_err() {
                    return Err(anyhow!("could not set Blocky API URL scheme to 'https'"));
                };
            }
            _ => {
                let err = anyhow!("Blocky API URL {} is not http or https", base_url);
                error!(%err);
                return Err(err);
            }
        }
        url.set_port(Some(api_port)).or(Err(anyhow!(
            "could not set API port to API URL -> is the URL valid? {base_url}"
        )))?;
        // reset path if any
        url.set_path("");
        let api = ApiClient {
            url,
            dns_port,
            api_port,
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        };
        debug!("created new API client: {api:?}");
        Ok(api)
    }

    pub async fn post_dnsquery(&self, query: DNSQuery) -> Result<DNSResponse> {
        debug!("posting DNS query: {query:?}");
        let url = self.url.join("api/query")?;
        let resp = self
            .client
            .post(url.to_string())
            .header("Content-Type", "application/json")
            .json(&query)
            .send()
            .await?
            .json::<DNSResponse>()
            .await?;
        debug!("received DNS response: {resp:?}");
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::ApiClient;
    use anyhow::Result;

    #[test]
    fn test_domain_name_parsing() -> Result<()> {
        let api = ApiClient::new("https://dns.test.com", 4000, 53)?;
        assert_eq!(
            api.url.to_string(),
            "https://dns.test.com:4000/",
            "check if URL parsing works for https://dns.test.com"
        );

        let api = ApiClient::new("https://dns.test.com:1234", 4000, 53)?;
        assert_eq!(
            api.url.to_string(),
            "https://dns.test.com:4000/",
            "check if URL parsing works for https://dns.test.com"
        );

        let api = ApiClient::new("https://dns.test.com/api", 4000, 53)?;
        assert_eq!(
            api.url.to_string(),
            "https://dns.test.com:4000/",
            "check if URL parsing works for https://dns.test.com/api"
        );

        let api = ApiClient::new("https://dns.test.com:1234/api", 4000, 53)?;
        assert_eq!(
            api.url.to_string(),
            "https://dns.test.com:4000/",
            "check if URL parsing works for https://dns.test.com:4000/api"
        );

        Ok(())
    }
}
