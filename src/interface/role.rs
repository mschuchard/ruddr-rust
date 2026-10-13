//! # Role
//!
//! `interface::role` consists of functions for interfacing with the Ruddr Role endpoints.
use crate::client::client;
use crate::model::role;
use crate::model::types;

/// Retrieves a specific Ruddr Role object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/projects/get-a-project-role.md)
/// ```ignore
/// let role = role(&client, types::UUID::try_from("7ad5a34a-07b7-48e9-a760-bd220d52e354").expect("uuid conversion failed")).await?;
/// ```
pub async fn role(client: &client::Client, id: types::UUID) -> Result<role::Role, reqwest::Error> {
    // retrieve role
    client
        .read::<role::Role>(&format!("project-roles/{id}"), None)
        .await
}

/// Retrieves the Ruddr Role objects by filters, and deserializes it to the corresponding vector of model structs. Default value for limit is 100.
/// [API Documentation](https://docs.ruddr.io/api-reference/project-roles/list-project-roles.md)
/// ```ignore
/// let roles = roles(
///     &client,
///     Some(types::UUID::try_from("095e0780-48bf-472c-8deb-2fc3ebc7d90c").expect("uuid conversion failed")),
///     None,
///     None,
///     None,
/// ).await?;
/// ```
pub async fn roles(
    client: &client::Client,
    project: Option<types::UUID>,
    starting_after: Option<types::UUID>,
    ending_before: Option<types::UUID>,
    limit: Option<u8>,
) -> Result<role::Roles, reqwest::Error> {
    // initialize params with limit
    let limit = limit.unwrap_or(100).to_string();
    let mut params = vec![("limit", limit.as_str())];

    // optional parameters for LIST
    if let Some(project) = &project {
        params.push(("projectId", project.into()));
    }
    if let Some(starting_after) = &starting_after {
        params.push(("startingAfter", starting_after.into()));
    }
    if let Some(ending_before) = &ending_before {
        params.push(("endingBefore", ending_before.into()));
    }

    // retrieve roles
    client
        .read::<role::Roles>("project-roles", Some(&params))
        .await
}

#[cfg(test)]
mod tests;
