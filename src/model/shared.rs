//! # Shared
//!
//! This module contains model structs and enums shared across the Ruddr API objects.
use crate::model::types;
use serde::{Deserialize, Serialize};

// structs
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: types::UUID,
    pub name: String,
    pub client: Entity,
}

// Simple generic entity struct for models comprised of only an ID and a name.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: types::UUID,
    pub name: String,
}

// enums
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CostMethod {
    Hourly,
    FixedHourly,
    FixedMonthly,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RecordStatus {
    Active,
    Archived,
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
