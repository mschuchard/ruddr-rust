//! # Expense Item
//!
//! `interface::expense_item` consists of functions for interfacing with the Ruddr Expense Item endpoints.
use std::fmt::Write;

use crate::client::client;
use crate::model::{expense_item, types};

/// Retrieves a specific Ruddr Expense Item object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://ruddr.readme.io/reference/get-an-expense-item)
/// ```ignore
/// let expense_item = expense_item(&client, types::UUID::try_from("77f5ccdc-4226-4ff1-877e-5644d0a04522").expect("invalid UUID")).await?;
/// ```
pub async fn expense_item(
    client: &client::Client,
    id: types::UUID,
) -> Result<expense_item::ExpenseItem, Box<dyn std::error::Error>> {
    // retrieve expense item
    Ok(client
        .read::<expense_item::ExpenseItem>(&format!("expense-items/{id}"), None)
        .await?)
}

/// Retrieves all Ruddr Expense Item objects, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://ruddr.readme.io/reference/list-expense-items)
/// ```ignore
/// let expense_items = expense_items(&client, Some(types::UUID::try_from("2bdab00d-86fb-46dc-ae05-7cc9c4aedc80").expect("invalid UUID"))).await?;
/// ```
pub async fn expense_items(
    client: &client::Client,
    expense_report: Option<types::UUID>,
) -> Result<expense_item::ExpenseItems, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("limit=100");

    // optional parameters for LIST
    if let Some(expense_report) = expense_report {
        write!(params, "&expenseReportId={}", expense_report)?;
    }

    // retrieve expense items
    Ok(client
        .read::<expense_item::ExpenseItems>("expense-items", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
