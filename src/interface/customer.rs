//! # Customer
//!
//! `interface::customer` consists of functions for interfacing with the Ruddr Client endpoints.
use crate::client::client;
use crate::model::customer;
use crate::model::types;

/// Retrieves a specific Ruddr Client object by id, and deserializes it to the corresponding model struct.
/// https://ruddr.readme.io/reference/get-a-client
/// ```ignore
/// let customer = customer(&client, Some(types::UUID::from("4cacdf11-71d1-4fbb-90ee-b091803581b0"))).await?;
/// ```
pub async fn customer(
    client: &client::Client,
    id: types::UUID,
) -> Result<customer::Client, Box<dyn std::error::Error>> {
    // retrieve client
    Ok(client
        .read::<customer::Client>(&format!("clients/{id}"), "")
        .await?)
}

/// Retrieves all Ruddr Client objects by filters, and deserializes it to the corresponding vector of model structs.
/// https://ruddr.readme.io/reference/list-clients
/// ```ignore
/// let customers = customers(
///     &client,
///     Some("JOE"),
/// ).await?;
/// ```
pub async fn customers(
    client: &client::Client,
    code: Option<&str>,
) -> Result<customer::Clients, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if code.is_some() {
        params = format!("{params}&code={}", code.unwrap())
    }

    // retrieve clients
    Ok(client.read::<customer::Clients>("clients", &params).await?)
}

#[cfg(test)]
mod tests;
