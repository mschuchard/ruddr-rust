//! # Role
//!
//! `interface::role` consists of functions for interfacing with the Ruddr Role endpoints.
use std::fmt::Write;

use crate::client::client;
use crate::model::role;
use crate::model::{enums, types};

/// Retrieves a specific Ruddr Role object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://ruddr.readme.io/reference/get-a-project-role)
/// ```ignore
/// let role = role(&client, types::UUID::try_from("7ad5a34a-07b7-48e9-a760-bd220d52e354").expect("uuid conversion failed")).await?;
/// ```
pub async fn role(
    client: &client::Client,
    id: types::UUID,
) -> Result<role::Role, Box<dyn std::error::Error>> {
    // retrieve role
    Ok(client
        .read::<role::Role>(&format!("project-roles/{id}"), None)
        .await?)
}

/// Retrieves all Ruddr Role objects by filters, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://ruddr.readme.io/reference/list-project-roles)
/// ```ignore
/// let roles = roles(
///     &client,
///     Some(types::UUID::try_from("095e0780-48bf-472c-8deb-2fc3ebc7d90c").expect("uuid conversion failed")),
/// ).await?;
/// ```
pub async fn roles(
    client: &client::Client,
    project: Option<types::UUID>,
) -> Result<role::Roles, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if let Some(project) = project {
        write!(params, "&projectId={}", project)?;
    }

    // retrieve roles
    Ok(client
        .read::<role::Roles>("project-roles", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
