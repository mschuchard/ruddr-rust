//! # Member
//!
//! `interface::member` consists of functions for interfacing with the Ruddr Member endpoints.
use crate::client::client;
use crate::model::{member, types};

/// Retrieves a specific Ruddr Workspace Member object by id, and deserializes it to the corresponding model struct.
/// https://ruddr.readme.io/reference/get-a-member
/// ```ignore
/// let member = member(&client, types::UUID::from("3f3df320-dd95-4a42-8eae-99243fb2ea86")).await?;
/// ```
pub async fn member(
    client: &client::Client,
    id: types::UUID,
) -> Result<member::Member, Box<dyn std::error::Error>> {
    // retrieve member
    Ok(client
        .read::<member::Member>(&format!("members/{id}"), "")
        .await?)
}

/// Retrieves all Ruddr Workspace Member objects, and deserializes it to the corresponding vector of model structs.
/// https://ruddr.readme.io/reference/list-members
/// ```ignore
/// let members = members(&client, Some("Joe"), Some("foo@bar.com")).await?;
/// ```
pub async fn members(
    client: &client::Client,
    name_contains: Option<&str>,
    email_contains: Option<&str>,
) -> Result<member::Members, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if name_contains.is_some() {
        params = format!("{params}&nameContains={}", name_contains.unwrap())
    }
    if email_contains.is_some() {
        params = format!("{params}&emailContains={}", email_contains.unwrap())
    }

    // retrieve members
    Ok(client.read::<member::Members>("members", &params).await?)
}

#[cfg(test)]
mod tests;
