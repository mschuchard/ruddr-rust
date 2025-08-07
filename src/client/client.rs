//! # Client
//!
//! `client::client` consists of functions for initializing Ruddr API clients, and initiating requests with those clients.
use log;
use reqwest;
use serde::de;
use std::env;

use super::request;

/// Client struct for reuse with various and multiple requests without explicit reqwest type usage.
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Instantiate a reusable Ruddr client through a constructor that returns a wrapped `Client` struct and boxed error.
    /// ```ignore
    /// // token as environment variable
    /// unsafe { std::env::set_var("RUDDR_TOKEN", "abcdefghi123456789"); }
    /// let client = Client::new(None)?;
    /// // token as parameter value
    /// let client = Client::new(Some("abcdefghi123456789"))?;
    /// ```
    pub fn new(token: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
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
                Err(_) => {
                    return Err(Box::from(
                        "ruddr api token was not input through code or RUDDR_TOKEN environment variable",
                    ));
                }
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

    /// Retrieves (GET) a specific Ruddr generic object by id, or specific generic objects by filters, and deserializes it/them to the corresponding struct/structs.
    /// This is public only to interface at the moment, but is abstract enough that assistance is super helpful to future me, and so documentation exists here.
    /// ```ignore
    /// let client = Client::new(Some("abcdefghi123456789"))?;
    /// let deser_response_read = client.read::<project::Project>(
    ///     "projects/095e0780-48bf-472c-8deb-2fc3ebc7d90c",
    ///     "",
    /// ).await?;
    /// let deser_response_list = client.read::<project::Projects>(
    ///     "projects",
    ///     "limit=100",
    /// ).await?;
    /// ```
    pub(crate) async fn read<M: de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<&str>,
    ) -> Result<M, Box<dyn std::error::Error>> {
        // construct and assign client request
        let request = request::Request::new(endpoint, params)?;
        log::debug!("request is {:?}", request);

        // retrieve object and deser
        let response = request.get(&self.client).await?;
        let deser = match response.error_for_status() {
            // deser if successful
            Ok(response) => response.json::<M>().await?,
            // provide information if failure
            Err(error) => {
                log::error!("request failed with status {:?}", error.status().unwrap());
                log::error!("{error}");
                return Err(Box::from("client read response failed"));
            }
        };

        log::debug!("successful read from Ruddr API");
        Ok(deser)
    }
}

#[cfg(test)]
mod tests;
