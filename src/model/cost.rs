//! # Cost
//!
//! `model::cost` is a model for the Ruddr Cost period object. This module is not publically accessible, but the structs and members are public for reading from `interface::cost` returns.
//! [API Documentation](https://docs.ruddr.io/api-reference/cost-periods/get-a-cost-period.md)
use crate::model::types;
use serde::{Deserialize, Serialize};

/// Model for Costs used with List operations.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Costs {
    pub results: Vec<Cost>,
    pub has_more: bool,
}

/// Model for Cost used with Read operations.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub id: types::UUID,
    pub is_default: bool,
    pub start: types::Date,
    pub end: Option<types::Date>,
    pub currency: String,
    pub cost_method_id: CostMethod,
    pub cost_per_hour: Option<f64>,
    pub overhead_cost_per_hour: Option<f64>,
    pub total_cost_per_hour: Option<f64>,
    pub cost_per_month: Option<f64>,
    pub overhead_cost_per_month: Option<f64>,
    pub total_cost_per_month: Option<f64>,
    pub created_at: types::Timestamp,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CostMethod {
    Hourly,
    FixedHourly,
    FixedMonthly,
}

#[cfg(test)]
mod tests;
