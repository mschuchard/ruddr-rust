//! # Allocation
//!
//! `interface::allocation` consists of functions for interfacing with the Ruddr Allocation endpoints.
use crate::client::client;
use crate::model::allocation;
use crate::model::{enums, types};

/// Retrieves a specific Ruddr Allocation object by id, and deserializes it to the corresponding model struct.
/// https://ruddr.readme.io/reference/get-an-allocation
/// ```ignore
/// let allocation = allocation(&client, types::UUID::from("212b8272-ed2a-4a91-950a-8a06b3546144")).await?;
/// ```
pub async fn allocation(
    client: &client::Client,
    id: types::UUID,
) -> Result<allocation::Allocation, Box<dyn std::error::Error>> {
    // retrieve client
    Ok(client
        .read::<allocation::Allocation>(&format!("allocations/{id}"), "")
        .await?)
}

/// Retrieves all Ruddr Allocation objects by filters, and deserializes it to the corresponding vector of model structs.
/// https://ruddr.readme.io/reference/list-allocations
/// ```ignore
/// let allocations = allocations(
///     &client,
///     Some(enums::AssignmentType::Project),
///     Some(types::UUID::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
///     Some(types::Date::from("2024-01-01")),
///     Some(types::Date::from("2024-01-01")),
/// ).await?;
/// ```
pub async fn allocations(
    client: &client::Client,
    assignment_type_id: Option<enums::AssignmentType>,
    member_id: Option<types::UUID>,
    start_date: Option<types::Date>,
    end_date: Option<types::Date>,
) -> Result<allocation::Allocations, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if assignment_type_id.is_some() {
        params = format!("{params}&assignmentTypeId={}", assignment_type_id.unwrap())
    }
    if member_id.is_some() {
        params = format!("{params}&memberId={}", member_id.unwrap())
    }
    if start_date.is_some() {
        params = format!("{params}&startOnBefore={}", start_date.unwrap())
    }
    if end_date.is_some() {
        params = format!("{params}&endOnAfter={}", end_date.unwrap())
    }

    // retrieve clients
    Ok(client
        .read::<allocation::Allocations>("allocations", &params)
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation() {
        let test = async {
            let client = client::Client::new(Some("abcdefghi123456789"))
                .expect("client with token could not be constructed");
            assert_eq!(
                allocation(
                    &client,
                    types::UUID::from("212b8272-ed2a-4a91-950a-8a06b3546144")
                )
                .await
                .unwrap_err()
                .to_string(),
                "client read response failed",
                "allocation retrieval did not fail on auth",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_allocations() {
        let test = async {
            let client = client::Client::new(Some("abcdefghi123456789"))
                .expect("client with token could not be constructed");
            assert_eq!(
                allocations(
                    &client,
                    Some(enums::AssignmentType::Project),
                    Some(types::UUID::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
                    Some(types::Date::from("2024-01-01")),
                    Some(types::Date::from("2024-01-01")),
                )
                .await
                .unwrap_err()
                .to_string(),
                "client read response failed",
                "allocations retrieval did not fail on auth",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
