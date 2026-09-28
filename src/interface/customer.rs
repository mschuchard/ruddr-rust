//! # Customer
//!
//! `interface::customer` consists of functions for interfacing with the Ruddr Client endpoints. This module and base Read function are named differently from the endpoint so as to avoid naming collisions with the API client module in external usage. All other code associated with this endpoint utilizes the endpoint name `client`.
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

/// Retrieves the Ruddr Client objects by filters, and deserializes it to the corresponding vector of model structs. Default value for limit is 100.
/// [API Documentation](https://docs.ruddr.io/api-reference/clients/list-clients.md)
/// ```ignore
/// let clients = clients(
///     &client,
///     Some("JOE"),
///     None,
///     None,
///     None,
/// ).await?;
/// ```
pub async fn clients(
    client: &client::Client,
    code: Option<&str>,
    starting_after: Option<model::types::UUID>,
    ending_before: Option<model::types::UUID>,
    limit: Option<u8>,
) -> Result<model::client::Clients, reqwest::Error> {
    // initialize params with limit
    let limit = limit.unwrap_or(100).to_string();
    let mut params = vec![("limit", limit.as_str())];

    // optional parameters for LIST
    if let Some(code) = &code {
        params.push(("code", code));
    }
    if let Some(starting_after) = &starting_after {
        params.push(("startingAfter", starting_after.into()));
    }
    if let Some(ending_before) = &ending_before {
        params.push(("endingBefore", ending_before.into()));
    }

    // retrieve clients
    Ok(client
        .read::<model::client::Clients>("clients", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
