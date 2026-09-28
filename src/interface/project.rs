//! # Project
//!
//! `interface::project` consists of functions for interfacing with the Ruddr Project endpoints.
use crate::client::client;
use crate::model::{project, types};

/// Retrieves a specific Ruddr Project object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/projects/get-a-project.md)
/// ```ignore
/// let project = project(&client, types::UUID::try_from("095e0780-48bf-472c-8deb-2fc3ebc7d90c").expect("invalid UUID")).await?;
/// ```
pub async fn project(
    client: &client::Client,
    id: types::UUID,
) -> Result<project::Project, reqwest::Error> {
    // retrieve project
    Ok(client
        .read::<project::Project>(&format!("projects/{id}"), None)
        .await?)
}

/// Retrieves the Ruddr Project objects by filters, and deserializes it to the corresponding vector of model structs. Default value for limit is 100.
/// [API Documentation](https://docs.ruddr.io/api-reference/projects/list-projects.md)
/// ```ignore
/// let projects = projects(
///     &client,
///     Some(types::UUID::try_from("d5afaffe-09e5-4d73-b02c-905b40fc6c22").expect("invalid UUID")),
///     Some(types::UUID::try_from("9b0927a6-35a1-4795-a4ca-10167b05f7de").expect("invalid UUID")),
///     Some(project::Status::InProgress),
///     Some("my_project"),
///     None,
///     None,
///     None,
/// ).await?;
/// ```
pub async fn projects(
    client: &client::Client,
    client_id: Option<types::UUID>,
    project_type: Option<types::UUID>,
    status: Option<project::Status>,
    name_contains: Option<&str>,
    starting_after: Option<types::UUID>,
    ending_before: Option<types::UUID>,
    limit: Option<u8>,
) -> Result<project::Projects, reqwest::Error> {
    // initialize params with limit
    let limit = limit.unwrap_or(100).to_string();
    let mut params = vec![("limit", limit.as_str())];

    // optional parameters for LIST
    if let Some(client_id) = &client_id {
        params.push(("clientId", client_id.into()));
    }
    if let Some(project_type) = &project_type {
        params.push(("projectTypeId={}", project_type.into()));
    }
    if let Some(status) = &status {
        params.push(("statusId", status.into()));
    }
    if let Some(name_contains) = &name_contains {
        params.push(("nameContains", name_contains));
    }
    if let Some(starting_after) = &starting_after {
        params.push(("startingAfter", starting_after.into()));
    }
    if let Some(ending_before) = &ending_before {
        params.push(("endingBefore", ending_before.into()));
    }

    // retrieve projects
    Ok(client
        .read::<project::Projects>("projects", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
