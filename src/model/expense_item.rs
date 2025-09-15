//! # Expense
//!
//! `model::expense_item` is a model for the Ruddr Expense Item object. This module is not publically accessible, but the structs and members are public for reading from `interface::expense_item` returns.
//! [API Documentation](https://ruddr.readme.io/reference/expense-item-object) (documentation has `Description` of `statusId` and `vendor` switched)
use crate::model::{shared, types};
use serde::{Deserialize, Serialize};

/// Model for ExpenseItems used with List operations.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseItems {
    pub results: Vec<ExpenseItem>,
    pub has_more: bool,
}

/// Model for ExpenseItem used with Read operations.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseItem {
    pub id: types::UUID,
    pub status_id: Status,
    pub vendor: String,
    pub notes: String,
    pub date: types::Date,
    pub currency: String,
    pub amount: f64,
    pub unit_count: Option<i64>,
    pub unit_amount: Option<f64>,
    pub is_reimbursable: bool,
    pub is_billable: bool,
    pub invoiced: bool,
    pub created_at: types::Timestamp,
    pub expense_report: ExpenseReport,
    pub expense_category: ExpenseCategory,
    pub member: shared::Entity,
    pub project: shared::Project,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseReport {
    pub id: types::UUID,
    pub title: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseCategory {
    pub id: types::UUID,
    pub name: String,
    pub unit_name: String,
}

// custom types: enum
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
