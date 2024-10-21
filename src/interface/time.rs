use reqwest;
use log;

use crate::client::request;
use crate::interface::util;
use crate::model::time;

// retrieve time entry GET
pub async fn time_entry(
    client: &reqwest::Client,
    id: &str,
) -> Result<time::TimeEntry, Box<dyn std::error::Error>> {
    log::debug!("retrieving time entry for {id}");

    // retrieve time entry and deser
    let time_entry = request::request(client, "time-entries", &format!("/{id}"))
        .await?
        .json::<time::TimeEntry>()
        .await?;

    log::debug!("time entry retrieved for {id}");
    Ok(time_entry)
}

// retrieve time entries LIST
pub async fn time_entries(
    client: &reqwest::Client,
    member: Option<&str>,
    project: Option<&str>,
    date: Option<&str>,
    begin_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<time::TimeEntries, Box<dyn std::error::Error>> {
    // construct params
    let mut params = String::from("?limit=100");

    // optional parameters for LIST
    if member.is_some() {
        let param = util::validate_uuid(member.unwrap())?;
        params = format!("&memberId={param}");
    }
    if project.is_some() {
        let param = util::validate_uuid(project.unwrap())?;
        params = format!("&projectId={param}")
    }
    if date.is_some() {
        let param = util::validate_date(date.unwrap())?;
        params = format!("&date={param}")
    }
    if begin_date.is_some() {
        let param = util::validate_date(begin_date.unwrap())?;
        params = format!("&dateOnAfter={param}")
    }
    if end_date.is_some() {
        let param = util::validate_date(end_date.unwrap())?;
        params = format!("&dataOnBefore={param}")
    }
    log::debug!("retrieving time entries with parameters {params}");

    // retrieve time entries and deser
    let time_entries = request::request(client, "time-entries", &params)
        .await?
        .json::<time::TimeEntries>()
        .await?;

    log::debug!("time entries retrieved");
    Ok(time_entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_entry() {
        let test = async {
            assert_eq!(
                time_entry(&reqwest::Client::new(), "ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
                    .await
                    .unwrap_err()
                    .to_string(),
                "error decoding response body",
                "member retrieval did not fail on json decoding",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_time_entries() {
        let test = async {
            assert_eq!(
                time_entries(
                    &reqwest::Client::new(),
                    Some("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885"),
                    Some("095e0780-48bf-472c-8deb-2fc3ebc7d90c"),
                    Some("2024-01-01"),
                    Some("2024-01-01"),
                    Some("2024-01-01"),
                )
                .await
                .unwrap_err()
                .to_string(),
                "error decoding response body",
                "member retrieval did not fail on json decoding",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
