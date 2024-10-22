use log;
use reqwest;

use crate::client::request;
use crate::interface::util;
use crate::model::project;

// retrieve project GET
pub async fn project(
    client: &reqwest::Client,
    id: &str,
) -> Result<project::Project, Box<dyn std::error::Error>> {
    log::debug!("retrieving project for {id}");

    // retrieve project and deser
    let project = request::request(client, "projects", &format!("/{id}"))
        .await?
        .json::<project::Project>()
        .await?;

    log::debug!("project retrieved for {id}");
    Ok(project)
}

// retrieve projects LIST
pub async fn projects(
    client: &reqwest::Client,
    client_id: Option<&str>,
    project_type: Option<&str>,
    status: Option<&str>,
) -> Result<project::Projects, Box<dyn std::error::Error>> {
    // construct params
    let mut params = String::from("?limit=100");

    // optional parameters for LIST
    if client_id.is_some() {
        let param = util::validate_uuid(client_id.unwrap())?;
        params = format!("&clientId={param}");
    }
    if project_type.is_some() {
        let param = util::validate_uuid(project_type.unwrap())?;
        params = format!("&projectTypeId={param}")
    }
    if status.is_some() {
        let param = util::validate_date(status.unwrap())?;
        params = format!("&statusId={param}")
    }
    log::debug!("retrieving projects with parameters {params}");

    // retrieve time entries and deser
    let projects = request::request(client, "projects", &params)
        .await?
        .json::<project::Projects>()
        .await?;

    log::debug!("projects retrieved");
    Ok(projects)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project() {
        let test = async {
            assert_eq!(
                project(
                    &reqwest::Client::new(),
                    "095e0780-48bf-472c-8deb-2fc3ebc7d90c"
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
    fn test_time_entries() {
        let test = async {
            assert_eq!(
                projects(
                    &reqwest::Client::new(),
                    Some("d5afaffe-09e5-4d73-b02c-905b40fc6c22"),
                    Some("9b0927a6-35a1-4795-a4ca-10167b05f7de"),
                    Some("in_progress"),
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
}
