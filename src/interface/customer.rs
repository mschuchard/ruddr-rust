//! # Customer
//!
//! `interface::customer` consists of functions for interfacing with the Ruddr Client endpoint.
use log;

use crate::client::client;
use crate::model::customer;
use crate::model::types;

/// Retrieves a specific Ruddr Client object by id, and deserializes it to the corresponding struct.
/// ```ignore
/// let customer = client(&client, Some(types::UUID::from("4cacdf11-71d1-4fbb-90ee-b091803581b0"))).await?;
/// ```
pub async fn client(
    client: &client::Client,
    id: types::UUID,
) -> Result<customer::Client, Box<dyn std::error::Error>> {
    log::debug!("retrieving client for {id}");

    // retrieve client and deser
    let customer = client
        .execute("clients", &format!("/{id}"))
        .await?
        .json::<customer::Client>()
        .await?;

    log::debug!("client retrieved for {id}");
    Ok(customer)
}

/// Retrieves all Ruddr Client objects by filters, and deserializes it to the corresponding vector of structs.
/// ```ignore
/// let customers = clients(
///     &client,
///     Some("JOE")),
/// ).await?;
/// ```
pub async fn time_entries(
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
