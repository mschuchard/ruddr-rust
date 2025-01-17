//! # Client
//!
//! `client::client` consists of functions for initializing Ruddr API clients and requests, and sending requests.
use crate::client::request;
use log;
use reqwest;
use std::env;

/// Client struct for reuse with various requests without explicit reqwest type usage
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Instantiate a reusable Ruddr client through a constructor that returns a wrapped `Client` struct and boxed error.
    /// ```ignore
    /// let client = Client::new(Some("abcdefghi123456789")).await?;
    /// ```
    pub async fn new(token: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        // initialize headers and establish json encoding
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        // determine authentication input method
        let bearer_token = match token {
            Some(token) => format!("Bearer {token}"),
            None => match env::var("RUDDR_TOKEN") {
                Ok(token) => format!("Bearer {token}"),
                Err(_) => return Err(Box::from(
                "ruddr api token was not input through code or RUDDR_TOKEN environment variable",
                )),
            },
        };
        // establish authentication and mark as sensitive
        let mut auth_token = reqwest::header::HeaderValue::from_str(&bearer_token)?;
        auth_token.set_sensitive(true);
        headers.insert(reqwest::header::AUTHORIZATION, auth_token);

        log::debug!("built headers are {:?}", headers);

        // build client
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        log::debug!("built client is {:?}", client);
        Ok(Self { client })
    }

    // execute request with client
    pub(crate) async fn execute(
        &self,
        endpoint: &str,
        params: &str,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        // construct and assign client request
        let request = request::Request::new(endpoint, params);

        // execute request and receive response
        log::debug!("initiating GET request at {}", request.url);
        // TODO: deser on generic struct like in Go?
        let response = self.client.get(request.url).send().await?;

        log::debug!("response received for GET request");
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let test = async {
            let client = Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            println!("client: {:?}", client)
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_client_new_env() {
        let test = async {
            // supposed to only be safe in single-thread
            std::env::set_var("RUDDR_TOKEN", "abcdefghi123456789");
            let client = Client::new(None)
                .await
                .expect("client with env token could not be constructed");
            println!("client: {:?}", client)
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_client_new_env_error() {
        let test = async {
            // supposed to only be safe in single-thread
            std::env::remove_var("RUDDR_TOKEN");
            assert_eq!(
                Client::new(None).await.unwrap_err().to_string(),
                "ruddr api token was not input through code or RUDDR_TOKEN environment variable",
                "attempted client build without token did not error expectedly",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_request() {
        let test = async {
            let client = Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with env token could not be constructed");
            let response = client
                .execute("members", "/3f3df320-dd95-4a42-8eae-99243fb2ea86")
                .await
                .expect("request transmission failed to receive a response");
            println!("response: {:?}", response);
            assert_eq!(
                response.status(),
                401,
                "the response did not return expected 401 status",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
