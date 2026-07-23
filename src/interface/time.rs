//! # Time
//!
//! `interface::time` consists of functions for interfacing with the Ruddr Time Entry endpoints.
use std::fmt::Write;

use crate::client::client;
use crate::model::{time, types};

/// Retrieves a specific Ruddr Time Entry object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/time-entries/get-a-time-entry.md)
/// ```ignore
/// let time_entry = time_entry(&client, types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885").expect("invalid UUID")).await?;
/// ```
pub async fn time_entry(
    client: &client::Client,
    id: types::UUID,
) -> Result<time::TimeEntry, reqwest::Error> {
    // retrieve time entry
    Ok(client
        .read::<time::TimeEntry>(&format!("time-entries/{id}"), None)
        .await?)
}

/// Retrieves the first 100 Ruddr Time Entry objects by filters, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://docs.ruddr.io/api-reference/time-entries/list-time-entries.md)
/// ```ignore
/// let time_entries = time_entries(
///     &client,
///     Some(types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885").expect("invalid UUID")),
///     Some(types::UUID::try_from("095e0780-48bf-472c-8deb-2fc3ebc7d90c").expect("invalid UUID")),
///     Some(types::UUID::try_from("c4b8d9e6-1f0a-4f5f-9d2e-8e4c8b0a1f2d").expect("invalid UUID")),
///     Some(time::Type::ProjectTime),
///     Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
///     Some(types::Date::try_from("2024-01-02").expect("date conversion failed")),
///     Some(types::Date::try_from("2024-01-03").expect("date conversion failed")),
///     None,
///     None,
///     None,
///     None,
/// ).await?;
/// ```
pub async fn time_entries(
    client: &client::Client,
    member: Option<types::UUID>,
    project: Option<types::UUID>,
    timesheet: Option<types::UUID>,
    time_type: Option<time::Type>,
    date: Option<types::Date>,
    date_after: Option<types::Date>,
    begin_date: Option<types::Date>,
    date_before: Option<types::Date>,
    end_date: Option<types::Date>,
    starting_after: Option<types::UUID>,
    ending_before: Option<types::UUID>,
) -> Result<time::TimeEntries, reqwest::Error> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if let Some(member) = member {
        write!(params, "&memberId={}", member).unwrap();
    }
    if let Some(project) = project {
        write!(params, "&projectId={}", project).unwrap();
    }
    if let Some(date) = date {
        write!(params, "&date={}", date).unwrap();
    }
    if let Some(date_after) = date_after {
        write!(params, "&dateAfter={}", date_after).unwrap();
    }
    if let Some(begin_date) = begin_date {
        write!(params, "&dateOnAfter={}", begin_date).unwrap();
    }
    if let Some(date_before) = date_before {
        write!(params, "&dateBefore={}", date_before).unwrap();
    }
    if let Some(end_date) = end_date {
        write!(params, "&dateOnBefore={}", end_date).unwrap();
    }
    if let Some(timesheet) = timesheet {
        write!(params, "&timesheetId={}", timesheet).unwrap();
    }
    if let Some(time_type) = time_type {
        write!(params, "&typeId={}", time_type).unwrap();
    }
    if let Some(starting_after) = starting_after {
        write!(params, "&startingAfter={}", starting_after).unwrap();
    }
    if let Some(ending_before) = ending_before {
        write!(params, "&endingBefore={}", ending_before).unwrap();
    }

    // retrieve time entries
    Ok(client
        .read::<time::TimeEntries>("time-entries", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
