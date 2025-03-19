//! # Project
//!
//! `interface::project` consists of functions for interfacing with the Ruddr Project endpoint.
use crate::client::client;
use crate::model::project;
use crate::model::types;

/// Retrieves a specific Ruddr Project object by id, and deserializes it to the corresponding struct.
/// ```ignore
/// let project = project(&client, types::UUID::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")).await?;
/// ```
pub async fn project(
    client: &client::Client,
    id: types::UUID,
) -> Result<project::Project, Box<dyn std::error::Error>> {
    // retrieve project
    Ok(client
        .read::<project::Project>("projects", &format!("/{id}"))
        .await?)
}

/// Retrieves all Ruddr Project objects by filters, and deserializes it to the corresponding vector of structs.
/// ```ignore
/// let projects = projects(
///     &client,
///     Some(types::UUID::from("d5afaffe-09e5-4d73-b02c-905b40fc6c22")),
///     Some(types::UUID::from("9b0927a6-35a1-4795-a4ca-10167b05f7de")),
///     Some(project::Status::InProgress),
///     Some("my_project"),
/// ).await?;
/// ```
pub async fn projects(
    client: &client::Client,
    client_id: Option<types::UUID>,
    project_type: Option<types::UUID>,
    status: Option<project::Status>,
    name_contains: Option<&str>,
) -> Result<project::Projects, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("?limit=100");

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
        .read::<project::Projects>("projects", &params)
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project() {
        let test = async {
            let client = client::Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            assert_eq!(
                project(
                    &client,
                    types::UUID::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")
                )
                .await
                .unwrap_err()
                .to_string(),
                "error decoding response body",
                "project retrieval did not fail on json decoding",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_projects() {
        let test = async {
            let client = client::Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            assert_eq!(
                projects(
                    &client,
                    Some(types::UUID::from("d5afaffe-09e5-4d73-b02c-905b40fc6c22")),
                    Some(types::UUID::from("9b0927a6-35a1-4795-a4ca-10167b05f7de")),
                    Some(project::Status::InProgress),
                    Some("my_project"),
                )
                .await
                .unwrap_err()
                .to_string(),
                "error decoding response body",
                "projects retrieval did not fail on json decoding",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
