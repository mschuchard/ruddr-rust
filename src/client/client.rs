//! # Client
//!
//! `client::client` consists of functions for initializing Ruddr API clients and requests, and sending requests.
use log;
use std::env;

use reqwest;

/// Client struct for reuse without explicit reqwest type usage
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
                Ok(value) => value,
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

    /// Return a reference to the `reqwest::Client`` type `client`` member of the `Client` struct.
    /// ```ignore
    /// let reqwest_client = client.client();
    /// ```
    pub fn client(&self) -> &reqwest::Client {
        return &self.client;
    }

    // execute request
    pub(crate) async fn request(
        &self,
        endpoint: &str,
        params: &str,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        // establish full url
        let url = format!("https://www.ruddr.io/api/workspace/{endpoint}{params}");

        log::debug!("initiating GET request at {url}");

        // TODO: deser on generic struct like in Go?
        let response = self.client.get(url).send().await?;

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
    fn test_client_read() {
        let test = async {
            let client = Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            let reqwest_client = client.client();
            println!("reqwest client: {:?}", reqwest_client)
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
                .request("members", "/3f3df320-dd95-4a42-8eae-99243fb2ea86")
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
