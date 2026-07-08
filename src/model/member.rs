//! # Member
//!
//! `model::member` is a model for the Ruddr Member object. This module is not publically accessible, but the structs and members are public for reading from `interface::member` returns.
//! [API Documentation](https://docs.ruddr.io/api-reference/members/get-a-member.md)
use crate::model::{shared, types};
use serde::{Deserialize, Serialize};

/// Model for Members used with List operations.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Members {
    pub results: Vec<Member>,
    pub has_more: bool,
}

/// Model for Member used with Read operations.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub id: types::UUID,
    pub name: String,
    pub email: String,
    pub is_active: bool,
    pub is_billable: bool,
    pub login_enabled: bool,
    pub invitation_status_id: InvitationStatus,
    pub employment_type_id: EmploymentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_method_id: Option<CostMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_rate_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_start_date: Option<types::Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_end_date: Option<types::Date>,
    pub time_off_allowed: bool,
    pub allowed_time_off_types: AllowedTimeOffTypes,
    pub time_off_approval_mode: TimeOffApprovalMode,
    pub internal_expense_approval_mode: InternalExpenseApprovalMode,
    pub receive_missing_time_reminders: bool,
    pub unsubmitted_timesheet_reminders: bool,
    pub automatic_timesheet_submission_confirmation: bool,
    pub timesheet_capacity_policy: TimesheetCapacityPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    pub created_at: types::Timestamp,
    pub track_time_by_duration: bool,
    pub track_time_by_time_range: bool,
    pub security_role: shared::Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<shared::Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<shared::Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discipline: Option<shared::Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_unit: Option<shared::Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice: Option<shared::Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<shared::Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<shared::Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_off_approver: Option<shared::Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_expense_approver: Option<shared::Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holiday_schedule: Option<shared::Entity>,
    pub tags: Vec<shared::Entity>,
    pub certifications: Vec<shared::Entity>,
    pub skills: Vec<shared::Entity>,
    pub time_off_types: Vec<shared::Entity>,
    pub availability_periods: Vec<AvailabilityPeriod>,
    pub cost_periods: Vec<CostPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_target_periods: Option<Vec<UtilizationTargetPeriod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbid_timesheet_submission_when_below_capacity: Option<bool>,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityPeriod {
    pub id: types::UUID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<types::Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<types::Date>,
    pub hours_per_day: Vec<i64>,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CostPeriod {
    pub id: types::UUID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<types::Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<types::Date>,
    pub currency: String,
    pub cost_method_id: CostMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_hour: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhead_cost_per_hour: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost_per_hour: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_month: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhead_cost_per_month: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost_per_month: Option<f64>,
    pub currency_name: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UtilizationTargetPeriod {
    pub id: types::UUID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<types::Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<types::Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_percentage: Option<f64>,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InvitationStatus {
    NotInvited,
    Invited,
    Accepted,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EmploymentType {
    Employee,
    Contractor,
    Other,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CostMethod {
    Hourly,
    FixedHourly,
    FixedMonthly,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AllowedTimeOffTypes {
    All,
    Custom,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TimesheetCapacityPolicy {
    Unrestricted,
    Timesheet,
    Week,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TimeOffApprovalMode {
    Auto,
    Manager,
    Member,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InternalExpenseApprovalMode {
    Auto,
    Manager,
    Member,
}

#[cfg(test)]
mod tests;
