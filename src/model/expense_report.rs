//! # Expense
//!
//! `model::expense` is models for the Ruddr Expense Report and Expense Item objects. This module is not publically accessible, but the structs and members are public for reading from `interface::expense` returns.
//! https://ruddr.readme.io/reference/expense-report-object
use crate::model::types;
use serde::{Deserialize, Serialize};

/// Model for ExpenseReports used with List operations
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseReports {
    pub results: Vec<ExpenseReport>,
    pub has_more: bool,
}

/// Model for ExpenseReport used with Read operations
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseReport {
    pub id: types::UUID,
    pub number: i64,
    pub title: String,
    pub notes: String,
    pub date: types::Date,
    pub created_at: types::Timestamp,
    pub member: Member,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub id: types::UUID,
    pub name: String,
}

#[cfg(test)]
mod tests;
