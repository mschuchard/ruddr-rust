//! # Customer
//!
//! `interface::customer` consists of functions for interfacing with the Ruddr Client endpoints. This module and base Read function are named differently from the endpoint so as to avoid naming collisions with the API client module in external usage. All other code associated with this endpoint utilizes the endpoint name `client`.
use std::fmt::Write;

use crate::client::client;
use crate::model;

/// Retrieves a specific Ruddr Client object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://ruddr.readme.io/reference/get-a-client)
/// ```ignore
/// let customer = customer(&client, model::types::UUID::try_from("4cacdf11-71d1-4fbb-90ee-b091803581b0").expect("invalid UUID")).await?;
/// ```
pub async fn customer(
    client: &client::Client,
    id: model::types::UUID,
) -> Result<model::client::Client, Box<dyn std::error::Error>> {
    // retrieve client
    Ok(client
        .read::<model::client::Client>(&format!("clients/{id}"), None)
        .await?)
}

/// Retrieves all Ruddr Client objects by filters, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://ruddr.readme.io/reference/list-clients)
/// ```ignore
/// let clients = clients(
///     &client,
///     Some("JOE"),
/// ).await?;
/// ```
pub async fn clients(
    client: &client::Client,
    code: Option<&str>,
) -> Result<model::client::Clients, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if let Some(code) = code {
        write!(params, "&code={}", code)?;
    }

    // retrieve clients
    Ok(client
        .read::<model::client::Clients>("clients", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
