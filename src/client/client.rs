//! # Client
//!
//! `client::client` consists of functions for initializing Ruddr API clients and requests, and sending requests.
use log;
use reqwest;
use serde::de;
use std::env;

use super::request;
use crate::model::types;

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

    /// Retrieves (GET) a specific Ruddr generic object by id, and deserializes it to the corresponding struct.
    /// This is public only to interface at the moment, but is abstract enough that assistance is super helpful to future me, and so documentation exists.
    /// ```ignore
    /// let client = Client::new(Some("abcdefghi123456789")).await?;
    /// let deser_response = client.read::<project::Project>(
    ///     "projects",
    ///     types::UUID::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c"),
    ///     "project",
    /// ).await?;
    /// ```
    pub(crate) async fn read<M: de::DeserializeOwned>(
        &self,
        endpoint: &str,
        id: types::UUID,
    ) -> Result<M, Box<dyn std::error::Error>> {
        log::debug!("retrieving {endpoint} for {id}");

        // construct and assign client request
        let request = request::Request::new(endpoint, &format!("/{id}"));

        // retrieve object and deser
        let deser_response = request.get(&self.client).await?.json::<M>().await?;

        log::debug!("{endpoint} retrieved for {id}");
        Ok(deser_response)
    }

    // retrieves (GET) specific ruddr generic objects by id; see above read() for more information as this is very similar
    pub(crate) async fn list<M: de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &str,
    ) -> Result<M, Box<dyn std::error::Error>> {
        log::debug!("retrieving {endpoint}");

        // construct and assign client request
        let request = request::Request::new(endpoint, params);

        // retrieve object and deser
        let deser_response = request.get(&self.client).await?.json::<M>().await?;

        log::debug!("{endpoint} retrieved");
        Ok(deser_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::project;

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
                .expect("client with env token could not be constructed");
            assert_eq!(
                client
                    .read::<project::Project>(
                        "project",
                        types::UUID::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c"),
                    )
                    .await
                    .unwrap_err()
                    .to_string(),
                "error decoding response body",
                "read did not fail on json decoding"
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_client_list() {
        let test = async {
            let client = Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with env token could not be constructed");
            assert_eq!(
                client
                    .list::<project::Project>("projects", "?limit=100")
                    .await
                    .unwrap_err()
                    .to_string(),
                "error decoding response body",
                "list did not fail on json decoding"
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
