//! # Customer
//!
//! `interface::customer` consists of functions for interfacing with the Ruddr Client endpoints. This module and base Read function are named differently from the endpoint so as to avoid naming collisions with the API client module in external usage. All other code associated with this endpoint utilizes the endpoint name `client`.
use std::fmt::Write;

use crate::client::client;
use crate::model;

/// Retrieves a specific Ruddr Client object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/clients/get-a-client.md)
/// ```ignore
/// let customer = customer(&client, types::UUID::try_from("4cacdf11-71d1-4fbb-90ee-b091803581b0").expect("invalid UUID")).await?;
/// ```
pub async fn customer(
    client: &client::Client,
    id: model::types::UUID,
) -> Result<model::client::Client, reqwest::Error> {
    // retrieve client
    Ok(client
        .read::<model::client::Client>(&format!("clients/{id}"), None)
        .await?)
}

/// Retrieves the first 100 Ruddr Client objects by filters, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://docs.ruddr.io/api-reference/clients/list-clients.md)
/// ```ignore
/// let clients = clients(
///     &client,
///     Some("JOE"),
///     None,
///     None,
/// ).await?;
/// ```
pub async fn clients(
    client: &client::Client,
    code: Option<&str>,
    starting_after: Option<model::types::UUID>,
    ending_before: Option<model::types::UUID>,
) -> Result<model::client::Clients, reqwest::Error> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if let Some(code) = code {
        write!(params, "&code={}", code).unwrap();
    }
    if let Some(starting_after) = starting_after {
        write!(params, "&startingAfter={}", starting_after).unwrap();
    }
    if let Some(ending_before) = ending_before {
        write!(params, "&endingBefore={}", ending_before).unwrap();
    }

    // retrieve clients
    Ok(client
        .read::<model::client::Clients>("clients", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
