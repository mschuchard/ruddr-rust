//! # Request
//!
//! `client::request` consists of functions for constructing and executing requests against the Ruddr API.
use log;

// request struct for composing request structures
#[derive(Debug)]
pub(super) struct Request {
    pub(super) url: String,
}

impl Request {
    // request constructor with endpoint and params
    pub(super) fn new(endpoint: &str, params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // validate endpoint is not empty
        if endpoint.is_empty() {
            log::error!("endpoint '{endpoint}' is empty");
            return Err(Box::from("invalid empty endpoint"));
        }

        log::debug!("request endpoint is {endpoint} and params is {params}");

        // prefix params with "?" char if they are not empty
        if params.is_empty() {
            Ok(Self {
                url: format!("https://www.ruddr.io/api/workspace/{endpoint}"),
            })
        } else {
            Ok(Self {
                url: format!("https://www.ruddr.io/api/workspace/{endpoint}?{params}"),
            })
        }
    }

    // execute get request with client
    pub(super) async fn get(
        self,
        client: &reqwest::Client,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        // execute request and receive response
        log::debug!("initiating GET request at {}", self.url);
        let response = client.get(self.url).send().await?;

        log::debug!("response received for GET request");
        Ok(response)
    }
}

#[cfg(test)]
mod tests;
