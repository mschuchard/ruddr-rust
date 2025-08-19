//! # Time
//!
//! `model::time` is a model for the Ruddr Time Entry object. This module is not publically accessible, but the structs and members are public for reading from `interface::time` returns.
//! [API Documentation](https://ruddr.readme.io/reference/time-entry-object)
use crate::model::{shared, types};
use serde::{Deserialize, Serialize};

/// Model for TimeEntries used with List operations.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntries {
    pub results: Vec<TimeEntry>,
    pub has_more: bool,
}

/// Model for TimeEntry used with Read operations.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    pub id: types::UUID,
    pub type_id: Type,
    pub status_id: Status,
    pub date: types::Date,
    pub minutes: i64,
    pub timer_started_at: Option<types::Timestamp>,
    pub notes: String,
    pub is_billable: bool,
    pub invoiced: bool,
    pub created_at: types::Timestamp,
    pub member: shared::Member,
    pub project: Option<shared::Project>,
    pub role: Option<Role>,
    pub task: Option<Task>,
    pub time_off_type: Option<TimeOffType>,
    pub invoice: Option<Invoice>,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: types::UUID,
    pub name: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: types::UUID,
    pub name: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeOffType {
    pub id: types::UUID,
    pub name: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: types::UUID,
    pub number: String,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    ProjectTime,
    TimeOff,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    NotSubmitted,
    PendingApproval,
    Approved,
    Rejected,
}

#[cfg(test)]
mod tests;
