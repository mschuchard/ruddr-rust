//! # Allocation
//!
//! `interface::allocation` consists of functions for interfacing with the Ruddr Allocation endpoints.
use crate::client::client;
use crate::model::allocation;
use crate::model::types;

/// Retrieves a specific Ruddr Allocation object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/allocations/get-an-allocation.md)
/// ```ignore
/// let allocation = allocation(&client, types::UUID::try_from("212b8272-ed2a-4a91-950a-8a06b3546144").expect("uuid conversion failed")).await?;
/// ```
pub async fn allocation(
    client: &client::Client,
    id: types::UUID,
) -> Result<allocation::Allocation, reqwest::Error> {
    // retrieve allocation
    client
        .read::<allocation::Allocation>(&format!("allocations/{id}"), None)
        .await
}

/// Retrieves the Ruddr Allocation objects by filters, and deserializes it to the corresponding vector of model structs. Default value for limit is 100.
/// [API Documentation](https://docs.ruddr.io/api-reference/allocations/list-allocations.md)
/// ```ignore
/// let allocations = allocations(
///     &client,
///     Some(allocation::AssignmentType::Project),
///     Some(types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885").expect("uuid conversion failed")),
///     Some(types::UUID::try_from("095e0780-48bf-472c-8deb-2fc3ebc7d90c").expect("uuid conversion failed")),
///     Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
///     None,
///     Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
/// ).await?;
/// ```
pub async fn allocations(
    client: &client::Client,
    assignment_type: Option<allocation::AssignmentType>,
    member: Option<types::UUID>,
    project: Option<types::UUID>,
    start_date: Option<types::Date>,
    start_before: Option<types::Date>,
    end_date: Option<types::Date>,
    end_after: Option<types::Date>,
    modified_at_after: Option<types::Timestamp>,
    modified_at_on_after: Option<types::Timestamp>,
    modified_at_before: Option<types::Timestamp>,
    modified_at_on_before: Option<types::Timestamp>,
    starting_after: Option<types::UUID>,
    ending_before: Option<types::UUID>,
    limit: Option<u8>,
) -> Result<allocation::Allocations, reqwest::Error> {
    // convert enum to owned string so the borrow in params remains valid
    let assignment_type = assignment_type.map(|assignment_type| assignment_type.to_string());
    // initialize params with limit
    let limit = limit.unwrap_or(100).to_string();
    let mut params = vec![("limit", limit.as_str())];

    // optional parameters for LIST
    if let Some(assignment_type) = &assignment_type {
        params.push(("assignmentTypeId", assignment_type.as_str()));
    }
    if let Some(member) = &member {
        params.push(("memberId", member.into()));
    }
    if let Some(project) = &project {
        params.push(("projectId", project.into()));
    }
    if let Some(start_date) = &start_date {
        params.push(("endOnAfter", start_date.into()));
    }
    if let Some(start_before) = &start_before {
        params.push(("startBefore", start_before.into()));
    }
    if let Some(end_date) = &end_date {
        params.push(("startOnBefore", end_date.into()));
    }
    if let Some(end_after) = &end_after {
        params.push(("endAfter", end_after.into()));
    }
    if let Some(modified_at_after) = &modified_at_after {
        params.push(("modifiedAtAfter", modified_at_after.into()));
    }
    if let Some(modified_at_on_after) = &modified_at_on_after {
        params.push(("modifiedAtOnAfter", modified_at_on_after.into()));
    }
    if let Some(modified_at_before) = &modified_at_before {
        params.push(("modifiedAtBefore", modified_at_before.into()));
    }
    if let Some(modified_at_on_before) = &modified_at_on_before {
        params.push(("modifiedAtOnBefore", modified_at_on_before.into()));
    }
    if let Some(starting_after) = &starting_after {
        params.push(("startingAfter", starting_after.into()));
    }
    if let Some(ending_before) = &ending_before {
        params.push(("endingBefore", ending_before.into()));
    }

    // retrieve allocations
    client
        .read::<allocation::Allocations>("allocations", Some(&params))
        .await
}

#[cfg(test)]
mod tests;
