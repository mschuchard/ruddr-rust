//! # Customer
//!
//! `interface::customer` consists of functions for interfacing with the Ruddr Client endpoint.
use log;

use crate::client::client;
use crate::model::customer;
use crate::model::types;

use super::read;

/// Retrieves a specific Ruddr Client object by id, and deserializes it to the corresponding struct.
/// ```ignore
/// let customer = customer(&client, Some(types::UUID::from("4cacdf11-71d1-4fbb-90ee-b091803581b0"))).await?;
/// ```
pub async fn customer(
    client: &client::Client,
    id: types::UUID,
) -> Result<customer::Client, Box<dyn std::error::Error>> {
    Ok(read::read::<customer::Client>(client, "clients", id, "client").await?)
}

/// Retrieves all Ruddr Client objects by filters, and deserializes it to the corresponding vector of structs.
/// ```ignore
/// let customers = customers(
///     &client,
///     Some("JOE")),
/// ).await?;
/// ```
pub async fn customers(
    client: &client::Client,
    code: Option<&str>,
) -> Result<customer::Clients, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("?limit=100");

    // optional parameters for LIST
    if code.is_some() {
        params = format!("{params}&code={}", code.unwrap())
    }
    log::debug!("retrieving clients with parameters {params}");

    // retrieve clients and deser
    let customers = client
        .execute("clients", &params)
        .await?
        .json::<customer::Clients>()
        .await?;

    log::debug!("clients retrieved");
    Ok(customers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        let test = async {
            let client = client::Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            assert_eq!(
                customer(
                    &client,
                    types::UUID::from("4cacdf11-71d1-4fbb-90ee-b091803581b0")
                )
                .await
                .unwrap_err()
                .to_string(),
                "error decoding response body",
                "client retrieval did not fail on json decoding",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_customers() {
        let test = async {
            let client = client::Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            assert_eq!(
                customers(&client, Some("JOE"))
                    .await
                    .unwrap_err()
                    .to_string(),
                "error decoding response body",
                "clients retrieval did not fail on json decoding",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
