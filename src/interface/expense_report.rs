//! # Expense Report
//!
//! `interface::expense_report` consists of functions for interfacing with the Ruddr Expense Report endpoints.
use crate::client::client;
use crate::model::{expense_report, types};

/// Retrieves a specific Ruddr Expense Report object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://docs.ruddr.io/api-reference/expense-reports/get-an-expense-report.md)
/// ```ignore
/// let expense_report = expense_report(&client, types::UUID::try_from("2bdab00d-86fb-46dc-ae05-7cc9c4aedc80").expect("invalid UUID")).await?;
/// ```
pub async fn expense_report(
    client: &client::Client,
    id: types::UUID,
) -> Result<expense_report::ExpenseReport, reqwest::Error> {
    // retrieve expense report
    Ok(client
        .read::<expense_report::ExpenseReport>(&format!("expense-reports/{id}"), None)
        .await?)
}

/// Retrieves the Ruddr Expense Report objects, and deserializes it to the corresponding vector of model structs. Default value for limit is 100.
/// [API Documentation](https://docs.ruddr.io/api-reference/expense-reports/list-expense-reports.md)
/// ```ignore
/// let expense_reports = expense_reports(&client, None, None, None).await?;
/// ```
pub async fn expense_reports(
    client: &client::Client,
    starting_after: Option<types::UUID>,
    ending_before: Option<types::UUID>,
    limit: Option<u8>,
) -> Result<expense_report::ExpenseReports, reqwest::Error> {
    // initialize params with limit
    let limit = limit.unwrap_or(100).to_string();
    let mut params = vec![("limit", limit.as_str())];

    if let Some(starting_after) = &starting_after {
        params.push(("startingAfter", starting_after.into()));
    }
    if let Some(ending_before) = &ending_before {
        params.push(("endingBefore", ending_before.into()));
    }

    // retrieve expense reports
    Ok(client
        .read::<expense_report::ExpenseReports>("expense-reports", Some(&params))
        .await?)
}

#[cfg(test)]
mod tests;
