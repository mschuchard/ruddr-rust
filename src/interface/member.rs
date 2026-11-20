//! # Member
//!
//! `interface::member` consists of functions for interfacing with the Ruddr Member endpoints.
use crate::client::client;
use crate::client::error::ClientError;
use crate::model::{member, types};

/// Retrieves a specific Ruddr Workspace Member object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/members/get-a-member.md)
/// ```ignore
/// let member = member(&client, types::UUID::try_from("3f3df320-dd95-4a42-8eae-99243fb2ea86").expect("invalid UUID")).await?;
/// ```
pub async fn member(
    client: &client::Client,
    id: types::UUID,
) -> Result<member::Member, ClientError> {
    // retrieve member
    client
        .read::<member::Member>(&format!("members/{id}"), None)
        .await
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
) -> Result<member::Members, ClientError> {
    // initialize params with limit
    let limit = limit.unwrap_or(100).to_string();
    let mut params = vec![("limit", limit.as_str())];

    // optional parameters for LIST
    if let Some(name_contains) = &name_contains {
        params.push(("nameContains", name_contains));
    }
    if let Some(email_contains) = &email_contains {
        params.push(("emailContains", email_contains));
    }
    if let Some(starting_after) = &starting_after {
        params.push(("startingAfter", starting_after.into()));
    }
    if let Some(ending_before) = &ending_before {
        params.push(("endingBefore", ending_before.into()));
    }

    // retrieve members
    client
        .read::<member::Members>("members", Some(&params))
        .await
}

#[cfg(test)]
mod tests;
