//! # Time
//!
//! `interface::time` consists of functions for interfacing with the Ruddr Time Entry endpoints.
use crate::client::client;
use crate::model::{time, types};

/// Retrieves a specific Ruddr Time Entry object by id, and deserializes it to the corresponding model struct.
/// https://ruddr.readme.io/reference/get-a-time-entry
/// ```ignore
/// let time_entry = time_entry(&client, types::UUID::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")).await?;
/// ```
pub async fn time_entry(
    client: &client::Client,
    id: types::UUID,
) -> Result<time::TimeEntry, Box<dyn std::error::Error>> {
    // retrieve time entry
    Ok(client
        .read::<time::TimeEntry>(&format!("time-entries/{id}"), "")
        .await?)
}

/// Retrieves all Ruddr Time Entry objects by filters, and deserializes it to the corresponding vector of model structs.
/// https://ruddr.readme.io/reference/list-time-entries
/// ```ignore
/// let time_entries = time_entries(
///     &client,
///     Some(types::UUID::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
///     Some(types::UUID::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")),
///     Some(types::Date::from("2024-01-01")),
///     Some(types::Date::from("2024-01-01")),
///     Some(types::Date::from("2024-01-01")),
/// ).await?;
/// ```
pub async fn time_entries(
    client: &client::Client,
    member: Option<types::UUID>,
    project: Option<types::UUID>,
    date: Option<types::Date>,
    begin_date: Option<types::Date>,
    end_date: Option<types::Date>,
) -> Result<time::TimeEntries, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if member.is_some() {
        params = format!("{params}&memberId={}", member.unwrap())
    }
    if project.is_some() {
        params = format!("{params}&projectId={}", project.unwrap())
    }
    if date.is_some() {
        params = format!("{params}&date={}", date.unwrap())
    }
    if begin_date.is_some() {
        params = format!("{params}&dateOnAfter={}", begin_date.unwrap())
    }
    if end_date.is_some() {
        params = format!("{params}&dateOnBefore={}", end_date.unwrap())
    }

    // retrieve time entries
    Ok(client
        .read::<time::TimeEntries>("time-entries", &params)
        .await?)
}

#[cfg(test)]
mod tests;
