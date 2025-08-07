//! # Project
//!
//! `interface::project` consists of functions for interfacing with the Ruddr Project endpoints.
use crate::client::client;
use crate::model::{enums, project, types};

/// Retrieves a specific Ruddr Project object by id, and deserializes it to the corresponding model struct.
/// https://ruddr.readme.io/reference/get-a-project
/// ```ignore
/// let project = project(&client, types::UUID::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")).await?;
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
/// https://ruddr.readme.io/reference/list-projects
/// ```ignore
/// let projects = projects(
///     &client,
///     Some(types::UUID::from("d5afaffe-09e5-4d73-b02c-905b40fc6c22")),
///     Some(types::UUID::from("9b0927a6-35a1-4795-a4ca-10167b05f7de")),
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
    if client_id.is_some() {
        params = format!("{params}&clientId={}", client_id.unwrap())
    }
    if project_type.is_some() {
        params = format!("{params}&projectTypeId={}", project_type.unwrap())
    }
    if status.is_some() {
        params = format!("{params}&statusId={}", status.unwrap());
    }
    if name_contains.is_some() {
        params = format!("{params}&nameContains={}", name_contains.unwrap())
    }

    // retrieve projects
    Ok(client
        .read::<project::Projects>("projects", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
