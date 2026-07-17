//! # Cost
//!
//! `interface::cost` consists of functions for interfacing with the Ruddr Cost period endpoints.
use std::fmt::Write;

use crate::client::client;
use crate::model::{cost, types};

/// Retrieves a specific Ruddr Cost period object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/cost-periods/get-a-cost-period.md)
/// ```ignore
/// let cost = cost(&client, types::UUID::try_from("b3a100b0-8e71-4f39-9d96-32f11838aa8c").expect("invalid UUID")).await?;
/// ```
pub async fn cost(client: &client::Client, id: types::UUID) -> Result<cost::Cost, reqwest::Error> {
    // retrieve cost target period
    Ok(client
        .read::<cost::Cost>(&format!("cost-periods/{id}"), None)
        .await?)
}

/// Retrieves the first 100 Ruddr Cost period objects by filters, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://docs.ruddr.io/api-reference/cost-periods/list-cost-periods.md)
/// ```ignore
/// let costs = costs(
///     &client,
///     None,
///     None,
///     Some(types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885").expect("uuid conversion failed")),
/// ).await?;
/// ```
pub async fn costs(
    client: &client::Client,
    starting_after: Option<types::UUID>,
    ending_before: Option<types::UUID>,
    member: Option<types::UUID>,
) -> Result<cost::Costs, reqwest::Error> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if let Some(starting_after) = starting_after {
        write!(params, "&startingAfter={}", starting_after).unwrap();
    }
    if let Some(ending_before) = ending_before {
        write!(params, "&endingBefore={}", ending_before).unwrap();
    }
    if let Some(member) = member {
        write!(params, "&memberId={}", member).unwrap();
    }

    // retrieve cost target periods
    Ok(client
        .read::<cost::Costs>("cost-periods", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
