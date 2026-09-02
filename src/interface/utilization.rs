//! # Utilization
//!
//! `interface::utilization` consists of functions for interfacing with the Ruddr Utilization target period endpoints.
use std::fmt::Write;

use crate::client::client;
use crate::model::{types, utilization};

/// Retrieves a specific Ruddr Utilization target period object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/utilization-target-periods/get-a-utilization-target-period.md)
/// ```ignore
/// let utilization = utilization(&client, types::UUID::try_from("8e6d6316-5bc2-4135-b99c-f604f29051ab").expect("invalid UUID")).await?;
/// ```
pub async fn utilization(
    client: &client::Client,
    id: types::UUID,
) -> Result<utilization::Utilization, reqwest::Error> {
    // retrieve utilization target period
    Ok(client
        .read::<utilization::Utilization>(&format!("utilization-target-periods/{id}"), None)
        .await?)
}

/// Retrieves the Ruddr Utilization target period objects by filters, and deserializes it to the corresponding vector of model structs. Default value for limit is 100.
/// [API Documentation](https://docs.ruddr.io/api-reference/utilization-target-periods/list-utilization-target-periods.md)
/// ```ignore
/// let utilizations = utilizations(
///     &client,
///     Some(types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885").expect("invalid UUID")),
///     None,
///     None,
///     None,
/// ).await?;
/// ```
pub async fn utilizations(
    client: &client::Client,
    member: Option<types::UUID>,
    starting_after: Option<types::UUID>,
    ending_before: Option<types::UUID>,
    limit: Option<u8>,
) -> Result<utilization::Utilizations, reqwest::Error> {
    // initialize params with limit
    let limit = limit.unwrap_or(100);
    let mut params = format!("limit={limit}");

    // optional parameters for LIST
    if let Some(member) = member {
        write!(params, "&memberId={}", member).unwrap();
    }
    if let Some(starting_after) = starting_after {
        write!(params, "&startingAfter={}", starting_after).unwrap();
    }
    if let Some(ending_before) = ending_before {
        write!(params, "&endingBefore={}", ending_before).unwrap();
    }

    // retrieve utilization target periods
    Ok(client
        .read::<utilization::Utilizations>("utilization-target-periods", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
