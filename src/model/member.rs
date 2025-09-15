//! # Member
//!
//! `model::member` is a model for the Ruddr Member object. This module is not publically accessible, but the structs and members are public for reading from `interface::member` returns.
//! [API Documentation](https://ruddr.readme.io/reference/member-object)
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
    pub cost_method_id: CostMethod,
    pub default_rate: f64,
    pub default_rate_currency: String,
    pub active_start_date: types::Date,
    pub active_end_date: types::Date,
    pub time_off_allowed: bool,
    pub allowed_time_off_types: AllowedTimeOffTypes,
    pub time_off_approval_mode: TimeOffApprovalMode,
    pub receive_missing_time_reminders: bool,
    pub unsubmitted_timesheet_reminders: bool,
    pub timesheet_capacity_policy: TimesheetCapacityPolicy,
    pub internal_id: String,
    pub internal_notes: String,
    pub created_at: types::Timestamp,
    pub security_role: shared::Entity,
    pub job_title: shared::Entity,
    pub discipline: shared::Entity,
    pub practice: shared::Entity,
    pub location: shared::Entity,
    pub manager: shared::Entity,
    pub time_off_approver: shared::Entity,
    pub holiday_schedule: shared::Entity,
    pub tags: Vec<shared::Entity>,
    pub skills: Vec<shared::Entity>,
    pub time_off_types: Vec<shared::Entity>,
    pub availability_periods: Vec<AvailabilityPeriod>,
    pub cost_periods: Vec<CostPeriod>,
    pub utilization_target_periods: Vec<UtilizationTargetPeriod>,
    pub forbid_timesheet_submission_when_below_capacity: bool,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityPeriod {
    pub id: types::UUID,
    pub start: types::Date,
    pub end: types::Date,
    pub hours_per_day: Vec<i64>,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CostPeriod {
    pub id: types::UUID,
    pub start: types::Date,
    pub end: types::Date,
    pub currency: String,
    pub cost_per_hour: i64,
    pub overhead_cost_per_hour: i64,
    pub total_cost_per_hour: i64,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UtilizationTargetPeriod {
    pub id: types::UUID,
    pub start: types::Date,
    pub end: types::Date,
    pub target_percentage: i64,
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
    Fixed,
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

#[cfg(test)]
mod tests;
