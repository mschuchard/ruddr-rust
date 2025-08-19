//! # Utilization
//!
//! `model::utilization` is a model for the Ruddr Utilization target period object. This module is not publically accessible, but the structs and members are public for reading from `interface::utilization` returns.
//! [API Documentation](https://ruddr.readme.io/reference/utilization-target-period-object)
use crate::model::types;
use serde::{Deserialize, Serialize};

/// Model for Utilizations used with List operations.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Utilizations {
    pub results: Vec<Utilization>,
    pub has_more: bool,
}

/// Model for Utilization used with Read operations.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Utilization {
    pub id: types::UUID,
    pub start: types::Date,
    pub target_percentage: i64,
    pub created_at: types::Timestamp,
    pub is_default: bool,
    pub end: types::Date,
}

#[cfg(test)]
mod tests;
