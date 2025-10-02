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
    pub(super) fn new(
        endpoint: &str,
        params: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // validate endpoint is not empty
        if endpoint.is_empty() {
            log::error!("endpoint is empty");
            return Err(Box::from("invalid empty endpoint"));
        }

        // prefix params with "?" char if they are specified
        match params {
            Some(params) => {
                log::debug!("request endpoint is {endpoint} and params is {params}");
                Ok(Self {
                    url: Url::parse(&format!(
                        "https://www.ruddr.io/api/workspace/{endpoint}?{params}"
                    ))?,
                })
            }
            None => {
                log::debug!("request endpoint is {endpoint} and params is empty");
                Ok(Self {
                    url: Url::parse(&format!("https://www.ruddr.io/api/workspace/{endpoint}"))?,
                })
            }
        }
    }

    // execute get request with client
    pub(super) async fn get(
        &self,
        client: &reqwest::Client,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        // execute request and receive response
        log::debug!("initiating GET request at {}", self.url);
        let response = client.get(self.url.clone()).send().await?;

        log::debug!("response received for GET request");
        Ok(response)
    }
}

#[cfg(test)]
mod tests;
