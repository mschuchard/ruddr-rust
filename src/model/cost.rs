//! # Cost
//!
//! `model::cost` is a model for the Ruddr Cost period object. This module is not publically accessible, but the structs and members are public for reading from `interface::cost` returns.
//! https://ruddr.readme.io/reference/cost-period-object
use crate::model::types;
use serde::{Deserialize, Serialize};

/// Model for Costs used with List operations
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Costs {
    pub results: Vec<Cost>,
    pub has_more: bool,
}

/// Model for Cost used with Read operations
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub cost_per_hour: i64,
    pub id: types::UUID,
    pub start: types::Date,
    pub created_at: types::Timestamp,
    pub overhead_cost_per_hour: f64,
    pub currency: String,
    pub is_default: bool,
    pub end: types::Date,
    pub total_cost_per_hour: f64,
}

#[cfg(test)]
mod tests;
