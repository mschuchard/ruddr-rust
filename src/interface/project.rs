//! # Project
//!
//! `interface::project` consists of functions for interfacing with the Ruddr Project endpoints.
use std::fmt::Write;

use crate::client::client;
use crate::model::{enums, project, types};

/// Retrieves a specific Ruddr Project object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://ruddr.readme.io/reference/get-a-project)
/// ```ignore
/// let project = project(&client, types::UUID::try_from("095e0780-48bf-472c-8deb-2fc3ebc7d90c").expect("invalid UUID")).await?;
/// ```
pub async fn project(
    client: &client::Client,
    id: types::UUID,
) -> Result<project::Project, Box<dyn std::error::Error>> {
    // retrieve project
    Ok(client
        .read::<project::Project>(&format!("projects/{id}"), None)
        .await?)
}

/// Retrieves all Ruddr Project objects by filters, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://ruddr.readme.io/reference/list-projects)
/// ```ignore
/// let projects = projects(
///     &client,
///     Some(types::UUID::try_from("d5afaffe-09e5-4d73-b02c-905b40fc6c22").expect("invalid UUID")),
///     Some(types::UUID::try_from("9b0927a6-35a1-4795-a4ca-10167b05f7de").expect("invalid UUID")),
///     Some(enums::Status::InProgress),
///     Some("my_project"),
/// ).await?;
/// ```
pub async fn projects(
    client: &client::Client,
    client_id: Option<types::UUID>,
    project_type: Option<types::UUID>,
    status: Option<enums::Status>,
    name_contains: Option<&str>,
) -> Result<project::Projects, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if let Some(client_id) = client_id {
        write!(params, "&clientId={}", client_id)?;
    }
    if let Some(project_type) = project_type {
        write!(params, "&projectTypeId={}", project_type)?;
    }
    if let Some(status) = status {
        write!(params, "&statusId={}", status)?;
    }
    if let Some(name_contains) = name_contains {
        write!(params, "&nameContains={}", name_contains)?;
    }

    // retrieve projects
    Ok(client
        .read::<project::Projects>("projects", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
