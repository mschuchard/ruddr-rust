//! # Utilization
//!
//! `interface::utilization` consists of functions for interfacing with the Ruddr Utilization target period endpoints.
use std::fmt::Write;

use crate::client::client;
use crate::model::{types, utilization};

/// Retrieves a specific Ruddr Utilization target period object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://ruddr.readme.io/reference/get-a-utilization-target-period)
/// ```ignore
/// let utilization = utilization(&client, types::UUID::from("8e6d6316-5bc2-4135-b99c-f604f29051ab")).await?;
/// ```
pub async fn utilization(
    client: &client::Client,
    id: types::UUID,
) -> Result<utilization::Utilization, Box<dyn std::error::Error>> {
    // retrieve utilization target period
    Ok(client
        .read::<utilization::Utilization>(&format!("utilization-target-periods/{id}"), None)
        .await?)
}

/// Retrieves all Ruddr Utilization target period objects by filters, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://ruddr.readme.io/reference/list-utilization-target-periods)
/// ```ignore
/// let utilizations = utilizations(
///     &client,
///     Some(types::UUID::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
/// ).await?;
/// ```
pub async fn utilizations(
    client: &client::Client,
    member_id: Option<types::UUID>,
) -> Result<utilization::Utilizations, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if let Some(member_id) = member_id {
        write!(params, "&memberId={}", member_id)?;
    }

    // retrieve clients
    Ok(client
        .read::<utilization::Utilizations>("utilization-target-periods", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
