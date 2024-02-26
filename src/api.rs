use anyhow::Result;
use http::uri::Port;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;

use http::{uri::Authority, Uri};

#[derive(Debug)]
pub struct BlockyApi {
    /// Blocky API Base Url
    pub url: String,
    pub dns_port: u16,
    client: reqwest::blocking::Client,
}

#[derive(Debug, Serialize)]
pub struct DNSQueryR<S>
where
    S: Into<String>,
{
    pub query: S,
    pub query_type: S,
}

pub type DNSQuery = DNSQueryR<String>;

// FIX: use this enum instead of the string
//
// #[derive(Default, Debug, Serialize)]
// enum DNSType {
// TODO: add more query types
//
//     #[default]
//     A,
//     AAA,
//     CNAME,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct DNSResponse {
    reason: String,
    response: String,
    response_type: String,
    return_code: String,
}

impl BlockyApi {
    pub fn new<S>(base_url: S, dns_port: u16) -> Self
    where
        S: Into<String>,
    {
        let base_uri: Uri = base_url
            .into()
            .parse::<Uri>()
            .expect("could not parse DNS URL to URI");

        if base_uri.host().is_some() {
            let auth = Authority::from_str(base_uri.authority().unwrap().as_ref()).unwrap();
            let base_uri_str = Uri::builder()
                .scheme(base_uri.scheme_str().unwrap_or("https"))
                .authority(auth)
                .path_and_query("/api")
                .build()
                .expect("could not build URI")
                .to_string();
            BlockyApi {
                url: base_uri_str,
                dns_port,
                client: reqwest::blocking::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap(),
            }
        } else {
            panic!("could not read DNS hostname from URL")
        }
    }

    pub fn post_dnsquery(&self, query: DNSQuery) -> Result<DNSResponse> {
        let query_response = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&query)?)
            .send();
        match query_response {
            Ok(response) => match response.status().as_u16() {
                200 => Ok(response.json::<DNSResponse>()?),
                400 => {
                    panic!("bad request")
                }
                _ => {
                    panic!("received unknown status code")
                }
            },
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocky_api_new_sucess() {
        let api = BlockyApi::new("https://dns.test.com", 53);
        assert_eq!(
            api.url, "https://dns.test.com/api",
            "check if URL parsing works for https://dns.test.com"
        );

        let api = BlockyApi::new("https://dns.test.com/api", 53);
        assert_eq!(
            api.url, "https://dns.test.com/api",
            "check if URL parsing works for https://dns.test.com/api"
        );

        let api = BlockyApi::new("dns.test.com", 53);
        assert_eq!(
            api.url, "https://dns.test.com/api",
            "check if URL parsing works for dns.test.com"
        );

        let api = BlockyApi::new("9.9.9.9", 53);
        assert_eq!(
            api.url, "https://9.9.9.9/api",
            "check if URL parsing works for 9.9.9.9"
        );

        let api = BlockyApi::new("9.9.9.9:4000", 53);
        assert_eq!(
            api.url, "https://9.9.9.9:4000/api",
            "check if URL parsing works for 9.9.9.9:4000"
        );

        let api = BlockyApi::new("localhost:4000", 53);
        assert_eq!(
            api.url, "https://localhost:4000/api",
            "check if URL parsing works for localhost:4000"
        );

        // FIX:: Somehow these URLs is not properly parsed
        //
        // let api = BlockyApi::new("dns.test.com/api".to_string(), 53);
        // assert_eq!(
        //     api.base_url, "https://dns.test.com/api",
        //     "check if URL parsing works for dns.test.com/api"
        // );
        //
        // let api = BlockyApi::new("9.9.9.9/api".to_string(), 53);
        // assert_eq!(
        //     api.url, "https://9.9.9.9/api",
        //     "check if URL parsing works for 9.9.9.9/api"
        // );
        //
        // let api = BlockyApi::new("localhost:4000", 53);
        // assert_eq!(
        //     api.url, "https://localhost:4000/api",
        //     "check if URL parsing works for localhost:4000/api"
        // );
    }

    #[test]
    #[should_panic]
    fn test_blocky_api_new_failure() {
        BlockyApi::new("".to_string(), 53);
        BlockyApi::new("dns".to_string(), 53);
        BlockyApi::new("dns/api".to_string(), 53);
        BlockyApi::new("htttp://dns.test.com".to_string(), 53);
        BlockyApi::new("http:///dns.test.com".to_string(), 53);
        BlockyApi::new("http://dns.test.com//api".to_string(), 53);
    }
}
