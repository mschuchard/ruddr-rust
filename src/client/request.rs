//! # Request
//!
//! `client::request` consists of functions for constructing and executing requests against the Ruddr API.
use log;
use reqwest::Url;

// request struct for composing request structures
#[derive(Debug)]
pub(super) struct Request {
    url: Url,
}

impl Request {
    // request constructor with endpoint and params
    pub(super) fn new(endpoint: &str, params: Option<&str>) -> Self {
        // validate endpoint is not empty
        assert!(!endpoint.is_empty(), "endpoint must not be empty");

        // prefix params with "?" char if they are specified
        let url = match params {
            Some(params) => {
                log::debug!("request endpoint is {endpoint} and params is {params}");
                Url::parse(&format!(
                    "https://www.ruddr.io/api/workspace/{endpoint}?{params}"
                ))
                .unwrap()
            }
            None => {
                log::debug!("request endpoint is {endpoint} and params is empty");
                Url::parse(&format!("https://www.ruddr.io/api/workspace/{endpoint}")).unwrap()
            }
        };
        Self { url }
    }

    // execute get request with client
    pub(super) async fn get(
        &self,
        client: &reqwest::Client,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // execute request and receive response
        log::debug!("initiating GET request at {}", self.url);
        let response = client.get(self.url.as_str()).send().await?;

        log::debug!("response received for GET request");
        Ok(response)
    }
}

#[cfg(test)]
mod tests;
