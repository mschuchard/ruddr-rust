//! # Client
//!
//! `client::client` consists of functions for initializing Ruddr API clients, and initiating requests with those clients.
use log;
use reqwest;
use serde::de;
use std::env;

use super::error::ClientError;
use super::request;

/// Client struct for reuse with various and multiple requests without explicit reqwest type usage.
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Instantiate a reusable Ruddr client through a constructor that returns a wrapped `Client` struct and `ClientError`. Default timeout is sixty seconds.
    /// ```ignore
    /// // token as environment variable with two minute timeout
    /// unsafe { std::env::set_var("RUDDR_TOKEN", "abcdefghi123456789"); }
    /// let client = Client::new(None, Some(120))?;
    /// // token as parameter value
    /// let client = Client::new(Some("abcdefghi123456789"), None)?;
    /// ```
    pub fn new(token: Option<&str>, timeout: Option<u64>) -> Result<Self, ClientError> {
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
                Err(_) => return Err(ClientError::MissingToken),
            },
        };
        // establish authentication and mark as sensitive
        let mut auth_token = reqwest::header::HeaderValue::from_str(&bearer_token)?;
        auth_token.set_sensitive(true);
        headers.insert(reqwest::header::AUTHORIZATION, auth_token);

        log::debug!("built headers are {headers:?}");

        // determine timeout
        let timeout = match timeout {
            Some(timeout) => std::time::Duration::from_secs(timeout),
            None => std::time::Duration::from_secs(60),
        };

        log::debug!("timeout is {timeout:?}");

        // build client
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .build()
            .map_err(|error| ClientError::Build(error.without_url()))?;

        log::debug!("built client is {client:?}");
        Ok(Self { client })
    }

    /// Retrieves (GET) a specific Ruddr generic object by id, or specific generic objects by filters, and deserializes it/them to the corresponding struct/structs.
    /// This is public only to interface at the moment, but is abstract enough that assistance is super helpful to future me, and so documentation exists here.
    /// ```ignore
    /// let client = Client::new(Some("abcdefghi123456789"))?;
    /// let deser_response_read = client.read::<project::Project>(
    ///     "projects/095e0780-48bf-472c-8deb-2fc3ebc7d90c",
    ///     None,
    /// ).await?;
    /// let deser_response_list = client.read::<project::Projects>(
    ///     "projects",
    ///     Some(&[("limit", "100")]),
    /// ).await?;
    /// ```
    pub(crate) async fn read<Response: de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Response, ClientError> {
        // construct and assign client request
        let request = request::Request::new(endpoint, params);
        log::debug!("request is {request:?}");

        // retrieve response and status
        let response = request.get(&self.client).await?;
        let status = response.status();

        // preserve the api error body (e.g. validation message on a 400) before the response is consumed
        if !status.is_success() {
            let body = response.text().await.unwrap_or_else(|error| {
                log::warn!("response body of failed request could not be read: {error}");
                String::new()
            });
            log::error!("request failed with status {status:?}");
            log::error!("{body}");
            return Err(ClientError::Status { status, body });
        }

        // deser
        let deser = response.json::<Response>().await?;

        log::debug!("successful read from Ruddr API");
        Ok(deser)
    }
}

#[cfg(test)]
mod tests;
