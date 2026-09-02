//! # Member
//!
//! `interface::member` consists of functions for interfacing with the Ruddr Member endpoints.
use std::fmt::Write;

use crate::client::client;
use crate::model::{member, types};

/// Retrieves a specific Ruddr Workspace Member object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/members/get-a-member.md)
/// ```ignore
/// let member = member(&client, types::UUID::try_from("3f3df320-dd95-4a42-8eae-99243fb2ea86").expect("invalid UUID")).await?;
/// ```
pub async fn member(
    client: &client::Client,
    id: types::UUID,
) -> Result<member::Member, reqwest::Error> {
    // retrieve member
    Ok(client
        .read::<member::Member>(&format!("members/{id}"), None)
        .await?)
}

/// Retrieves the Ruddr Workspace Member objects, and deserializes it to the corresponding vector of model structs. Default value for limit is 100.
/// [API Documentation](https://docs.ruddr.io/api-reference/members/list-members.md)
/// ```ignore
/// let members = members(
///     &client,
///     Some("Joe"),
///     Some("foo@bar.com"),
///     None,
///     None,
///     None,
/// ).await?;
/// ```
pub async fn members(
    client: &client::Client,
    name_contains: Option<&str>,
    email_contains: Option<&str>,
    starting_after: Option<types::UUID>,
    ending_before: Option<types::UUID>,
    limit: Option<u8>,
) -> Result<member::Members, reqwest::Error> {
    // initialize params with limit
    let limit = limit.unwrap_or(100);
    let mut params = format!("limit={limit}");

    // optional parameters for LIST
    if let Some(name_contains) = name_contains {
        write!(params, "&nameContains={}", name_contains).unwrap();
    }
    if let Some(email_contains) = email_contains {
        write!(params, "&emailContains={}", email_contains).unwrap();
    }
    if let Some(starting_after) = starting_after {
        write!(params, "&startingAfter={}", starting_after).unwrap();
    }
    if let Some(ending_before) = ending_before {
        write!(params, "&endingBefore={}", ending_before).unwrap();
    }

    // retrieve members
    Ok(client
        .read::<member::Members>("members", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
