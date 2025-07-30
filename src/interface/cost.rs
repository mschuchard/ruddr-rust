//! # Cost
//!
//! `interface::cost` consists of functions for interfacing with the Ruddr Cost period endpoints.
use crate::client::client;
use crate::model::{cost, types};

/// Retrieves a specific Ruddr Cost period object by id, and deserializes it to the corresponding model struct.
/// https://ruddr.readme.io/reference/get-a-cost-period
/// ```ignore
/// let cost = cost(&client, types::UUID::from("b3a100b0-8e71-4f39-9d96-32f11838aa8c")).await?;
/// ```
pub async fn cost(
    client: &client::Client,
    id: types::UUID,
) -> Result<cost::Cost, Box<dyn std::error::Error>> {
    // retrieve cost target period
    Ok(client
        .read::<cost::Cost>(&format!("cost-periods/{id}"), "")
        .await?)
}

/// Retrieves all Ruddr Cost period objects by filters, and deserializes it to the corresponding vector of model structs.
/// https://ruddr.readme.io/reference/list-cost-periods
/// ```ignore
/// let costs = costs(
///     &client,
///     Some(types::UUID::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
/// ).await?;
/// ```
pub async fn costs(
    client: &client::Client,
    member_id: Option<types::UUID>,
) -> Result<cost::Costs, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if member_id.is_some() {
        params = format!("{params}&memberId={}", member_id.unwrap())
    }

    // retrieve clients
    Ok(client.read::<cost::Costs>("cost-periods", &params).await?)
}

#[cfg(test)]
mod tests;
