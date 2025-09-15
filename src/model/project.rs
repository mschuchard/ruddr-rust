//! # Project
//!
//! `model::project` is a model for the Ruddr Project object. This module is not publically accessible, but the structs and members are public for reading from `interface::project` returns.
//! [API Documentation](https://ruddr.readme.io/reference/project-object)
use crate::model::{enums, shared, types};
use serde::{Deserialize, Serialize};

/// Model for Projects used with List operations.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Projects {
    pub results: Vec<Project>,
    pub has_more: bool,
}

/// Model for Project used with Read operations.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: types::UUID,
    pub key: types::Slug,
    pub name: String,
    pub notes: String,
    pub status_id: enums::Status,
    pub start: Option<types::Date>,
    pub end: Option<types::Date>,
    pub code: String,
    pub po_number: String,
    pub billing_type_id: BillingType,
    pub is_billable: bool,
    pub currency: String,
    pub revenue_recognition_method: Option<RevenueRecognitionMethod>,
    pub fixed_fee: Option<i64>,
    pub fixed_recurring_fee: Option<i64>,
    pub fixed_recurring_start: Option<types::Date>,
    pub fixed_recurring_end: Option<types::Date>,
    pub use_roles: bool,
    pub use_budget: bool,
    pub budget_mode: Option<BudgetMode>,
    pub use_monthly_budget: bool,
    pub monthly_budget_mode: Option<MonthlyBudgetMode>,
    pub cap_max_member_hours_per_day: bool,
    pub max_member_hours_per_day: Option<i32>,
    pub cap_max_member_hours_per_week: bool,
    pub max_member_hours_per_week: Option<i32>,
    pub cap_max_member_hours_per_month: bool,
    pub max_member_hours_per_month: Option<i32>,
    pub requires_notes: bool,
    pub requires_tasks: bool,
    pub record_status_id: RecordStatus,
    pub is_productive: Option<bool>,
    pub lock_time_and_expenses: bool,
    pub track_time_to_assigned_roles: bool,
    pub created_at: types::Timestamp,
    pub client: shared::Entity,
    pub practice: shared::Entity,
    pub project_type: shared::Entity,
    pub tags: Vec<shared::Entity>,
    pub sales_representative: Option<shared::Entity>,
    pub business_unit: Option<shared::Entity>,
    pub budget: Option<Budget>,
    pub monthly_budget: Option<MonthlyBudget>,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub revenue: i64,
    pub services_revenue: i64,
    pub other_revenue: i64,
    pub billable_expenses: i64,
    pub non_billable_expenses: i64,
    pub billable_hours: i64,
    pub non_billable_hours: i64,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyBudget {
    pub revenue: i64,
    pub services_revenue: i64,
    pub other_revenue: i64,
    pub billable_expenses: i64,
    pub non_billable_expenses: i64,
    pub billable_hours: i64,
    pub non_billable_hours: i64,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BillingType {
    #[serde(rename = "tm")]
    TimeAndMaterials,
    Fixed,
    FixedRecurring,
    NonBillable,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RevenueRecognitionMethod {
    Invoiced,
    Manual,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BudgetMode {
    Summary,
    Detailed,
    Aggregated,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MonthlyBudgetMode {
    Summary,
    Detailed,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RecordStatus {
    Active,
    Archived,
}

#[cfg(test)]
mod tests;
