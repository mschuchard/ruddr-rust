//! # Time
//!
//! `model::time` is a model for the Ruddr Time Entry object. This module is not publically accessible, but the structs and members are public for reading from `interface::time` returns.
//! [API Documentation](https://ruddr.readme.io/reference/time-entry-object)
use crate::model::{shared, types};
use serde::Deserialize;

/// Model for TimeEntries used with List operations.
#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntries {
    pub results: Vec<TimeEntry>,
    pub has_more: bool,
}

/// Model for TimeEntry used with Read operations.
#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    pub id: types::UUID,
    pub type_id: Type,
    pub status_id: Status,
    pub client_status_id: Option<ClientStatus>,
    pub date: types::Date,
    pub minutes: i64,
    pub timer_started_at: Option<types::Timestamp>,
    pub notes: String,
    pub is_billable: bool,
    pub invoiced: bool,
    pub rate_currency: Option<String>,
    pub rate: Option<f64>,
    pub cost_currency: Option<String>,
    pub cost_per_hour: Option<f64>,
    pub created_at: types::Timestamp,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub member: shared::Entity,
    pub project: Option<shared::Project>,
    pub role: Option<shared::Entity>,
    pub task: Option<shared::Entity>,
    pub time_off_type: Option<shared::Entity>,
    pub invoice: Option<Invoice>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: types::UUID,
    pub number: String,
    pub line: Line,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub id: types::UUID,
    pub number: i64,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    ProjectTime,
    TimeOff,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    NotSubmitted,
    PendingApproval,
    Approved,
    Rejected,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ClientStatus {
    Approved,
    Rejected,
}

#[cfg(test)]
mod tests;
