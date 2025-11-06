//! # Allocation
//!
//! `interface::allocation` consists of functions for interfacing with the Ruddr Allocation endpoints.
use std::fmt::Write;

use crate::client::client;
use crate::model::allocation;
use crate::model::{enums, types};

/// Retrieves a specific Ruddr Allocation object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://ruddr.readme.io/reference/get-an-allocation)
/// ```ignore
/// let allocation = allocation(&client, types::UUID::try_from("212b8272-ed2a-4a91-950a-8a06b3546144").expect("uuid conversion failed")).await?;
/// ```
pub async fn allocation(
    client: &client::Client,
    id: types::UUID,
) -> Result<allocation::Allocation, Box<dyn std::error::Error>> {
    // retrieve allocation
    Ok(client
        .read::<allocation::Allocation>(&format!("allocations/{id}"), None)
        .await?)
}

/// Retrieves all Ruddr Allocation objects by filters, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://ruddr.readme.io/reference/list-allocations)
/// ```ignore
/// let allocations = allocations(
///     &client,
///     Some(enums::AssignmentType::Project),
///     Some(types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885").expect("uuid conversion failed")),
///     Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
///     Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
/// ).await?;
/// ```
pub async fn allocations(
    client: &client::Client,
    assignment_type: Option<enums::AssignmentType>,
    member: Option<types::UUID>,
    start_date: Option<types::Date>,
    end_date: Option<types::Date>,
) -> Result<allocation::Allocations, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if let Some(assignment_type) = assignment_type {
        write!(params, "&assignmentTypeId={}", assignment_type)?;
    }
    if let Some(member) = member {
        write!(params, "&memberId={}", member)?;
    }
    if let Some(start_date) = start_date {
        write!(params, "&startOnBefore={}", start_date)?;
    }
    if let Some(end_date) = end_date {
        write!(params, "&endOnAfter={}", end_date)?;
    }

    // retrieve allocations
    Ok(client
        .read::<allocation::Allocations>("allocations", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
