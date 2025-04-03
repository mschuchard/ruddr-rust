//! # Allocation
//!
//! `model::allocation` is a model for the Ruddr Allocation object
use crate::model::types;
use serde::Deserialize;

/// Model for Allocations used with List operations
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocations {
    pub(crate) results: Vec<Allocation>,
    pub(crate) has_more: bool,
}

/// Model for Allocation used with Read operations
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub(crate) id: types::UUID,
    pub(crate) resource_type_id: ResourceType,
    pub(crate) assignment_type_id: AssignmentType,
    pub(crate) start: types::Date,
    pub(crate) end: types::Date,
    pub(crate) unit: Unit,
    pub(crate) hours_per_day: Option<i64>,
    pub(crate) hours_per_week: Option<i64>,
    pub(crate) hours_per_month: Option<i64>,
    pub(crate) total_hours: i64,
    pub(crate) is_billable: bool,
    pub(crate) notes: String,
    pub(crate) read_only: bool,
    pub(crate) entity: Entity,
    pub(crate) created_at: types::Timestamp,
    pub(crate) member: Option<Member>,
    pub(crate) placeholder: Option<Placeholder>,
    pub(crate) project: Option<Project>,
    pub(crate) role: Option<Role>,
    pub(crate) task: Option<Task>,
    pub(crate) time_off_type: Option<TimeOffType>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Member {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Placeholder {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Project {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
    pub(crate) client: Client,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Client {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Role {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Task {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TimeOffType {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ResourceType {
    Member,
    Placeholder,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AssignmentType {
    Project,
    TimeOff,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Unit {
    Day,
    Week,
    Month,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Entity {
    Allocation,
    TimeEntry,
    Holiday,
}
