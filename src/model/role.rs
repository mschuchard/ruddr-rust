//! # Project Role
//!
//! `model::role` is a model for the Ruddr Project Role object. This module is not publically accessible, but the structs and members are public for reading from `interface::role` returns.
//! [API Documentation](https://ruddr.readme.io/reference/project-role-object)
use crate::model::{shared, types};
use serde::Deserialize;

/// Model for Roles used with List operations.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roles {
    pub results: Vec<Role>,
    pub has_more: bool,
}

/// Model for Role used with Read operations.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: types::UUID,
    pub name: String,
    pub is_active: bool,
    pub is_billable: bool,
    pub rate: Option<f64>,
    pub created_at: types::Timestamp,
    pub project: shared::Project,
    pub discipline: shared::Entity,
    pub budget: Option<Budget>,
    pub monthly_budget: Option<Budget>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub billable_hours: Option<i64>,
    pub non_billable_hours: i64,
}

#[cfg(test)]
mod tests;
