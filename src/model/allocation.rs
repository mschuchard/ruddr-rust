//! # Allocation
//!
//! `model::allocation` is a model for the Ruddr Allocation object. This module is not publically accessible, but the structs and members are public for reading from `interface::allocation` returns.
//! [API Documentation](https://ruddr.readme.io/reference/allocation-object)
use crate::model::{enums, shared, types};
use serde::Deserialize;

/// Model for Allocations used with List operations.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocations {
    pub results: Vec<Allocation>,
    pub has_more: bool,
}

/// Model for Allocation used with Read operations.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub id: types::UUID,
    pub resource_type_id: ResourceType,
    pub assignment_type_id: enums::AssignmentType,
    pub start: types::Date,
    pub end: types::Date,
    pub unit: Unit,
    pub hours_per_day: Option<i64>,
    pub hours_per_week: Option<i64>,
    pub hours_per_month: Option<i64>,
    pub total_hours: i64,
    pub is_billable: bool,
    pub notes: String,
    pub read_only: bool,
    pub entity: Entity,
    pub created_at: types::Timestamp,
    pub member: Option<shared::Entity>,
    pub placeholder: Option<shared::Entity>,
    pub project: Option<shared::Project>,
    pub role: Option<shared::Entity>,
    pub task: Option<shared::Entity>,
    pub time_off_type: Option<shared::Entity>,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Member,
    Placeholder,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    Day,
    Week,
    Month,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Entity {
    Allocation,
    TimeEntry,
    Holiday,
}

#[cfg(test)]
mod tests;
