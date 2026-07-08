//! # Project Role
//!
//! `model::role` is a model for the Ruddr Project Role object. This module is not publically accessible, but the structs and members are public for reading from `interface::role` returns.
//! [API Documentation](https://docs.ruddr.io/api-reference/project-roles/get-a-project-role.md)
use crate::model::{shared, types};
use serde::{Deserialize, Serialize};

/// Model for Roles used with List operations.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Roles {
    pub results: Vec<Role>,
    pub has_more: bool,
}

/// Model for Role used with Read operations.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: types::UUID,
    pub name: String,
    pub is_active: bool,
    pub is_billable: bool,
    pub rate: Option<f64>,
    pub created_at: types::Timestamp,
    pub project: shared::Project,
    pub discipline: Option<shared::Entity>,
    pub practice: Option<shared::Entity>,
    pub location: Option<shared::Entity>,
    pub budget: Option<Budget>,
    pub monthly_budget: Option<Budget>,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub billable_hours: Option<i64>,
    pub non_billable_hours: Option<i64>,
}

#[cfg(test)]
mod tests;
