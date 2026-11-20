//! # Expense Item
//!
//! `interface::expense_item` consists of functions for interfacing with the Ruddr Expense Item endpoints.
use crate::client::client;
use crate::client::error::ClientError;
use crate::model::{expense_item, types};

/// Retrieves a specific Ruddr Expense Item object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/expense-items/get-an-expense-item.md)
/// ```ignore
/// let expense_item = expense_item(&client, types::UUID::try_from("77f5ccdc-4226-4ff1-877e-5644d0a04522").expect("invalid UUID")).await?;
/// ```
pub async fn expense_item(
    client: &client::Client,
    id: types::UUID,
) -> Result<expense_item::ExpenseItem, ClientError> {
    // retrieve expense item
    client
        .read::<expense_item::ExpenseItem>(&format!("expense-items/{id}"), None)
        .await
}

/// Retrieves the Ruddr Expense Item objects, and deserializes it to the corresponding vector of model structs. Default value for limit is 100.
/// [API Documentation](https://docs.ruddr.io/api-reference/expense-items/list-expense-items.md)
/// ```ignore
/// let expense_items = expense_items(
///     &client,
///     Some(types::UUID::try_from("2bdab00d-86fb-46dc-ae05-7cc9c4aedc80").expect("invalid UUID")),
///     None,
///     None,
///     None,
/// ).await?;
/// ```
pub async fn expense_items(
    client: &client::Client,
    expense_report: Option<types::UUID>,
    starting_after: Option<types::UUID>,
    ending_before: Option<types::UUID>,
    limit: Option<u8>,
) -> Result<expense_item::ExpenseItems, ClientError> {
    // initialize params with limit
    let limit = limit.unwrap_or(100).to_string();
    let mut params = vec![("limit", limit.as_str())];

    // optional parameters for LIST
    if let Some(expense_report) = &expense_report {
        params.push(("expenseReportId", expense_report.into()));
    }
    if let Some(starting_after) = &starting_after {
        params.push(("startingAfter", starting_after.into()));
    }
    if let Some(ending_before) = &ending_before {
        params.push(("endingBefore", ending_before.into()));
    }

    // retrieve expense items
    client
        .read::<expense_item::ExpenseItems>("expense-items", Some(&params))
        .await
}

#[cfg(test)]
mod tests;
