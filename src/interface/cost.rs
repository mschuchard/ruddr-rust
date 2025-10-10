//! # Cost
//!
//! `interface::cost` consists of functions for interfacing with the Ruddr Cost period endpoints.
use std::fmt::Write;

use crate::client::client;
use crate::model::{cost, types};

/// Retrieves a specific Ruddr Cost period object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://ruddr.readme.io/reference/get-a-cost-period)
/// ```ignore
/// let cost = cost(&client, types::UUID::try_from("b3a100b0-8e71-4f39-9d96-32f11838aa8c").expect("invalid UUID")).await?;
/// ```
pub async fn cost(
    client: &client::Client,
    id: types::UUID,
) -> Result<cost::Cost, Box<dyn std::error::Error>> {
    // retrieve cost target period
    Ok(client
        .read::<cost::Cost>(&format!("cost-periods/{id}"), None)
        .await?)
}

/// Retrieves all Ruddr Cost period objects by filters, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://ruddr.readme.io/reference/list-cost-periods)
/// ```ignore
/// let costs = costs(
///     &client,
///     Some(types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885").expect("uuid conversion failed")),
/// ).await?;
/// ```
pub async fn costs(
    client: &client::Client,
    member_id: Option<types::UUID>,
) -> Result<cost::Costs, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if let Some(member_id) = member_id {
        write!(params, "&memberId={}", member_id)?;
    }

    // retrieve clients
    Ok(client
        .read::<cost::Costs>("cost-periods", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
