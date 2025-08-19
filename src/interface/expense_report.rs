//! # Expense Report
//!
//! `interface::expense_report` consists of functions for interfacing with the Ruddr Expense Report endpoints.
use crate::client::client;
use crate::model::{expense_report, types};

/// Retrieves a specific Ruddr Expense Report object by id, and deserializes it to the corresponding model struct.
/// [API Documentation](https://ruddr.readme.io/reference/get-an-expense-report)
/// ```ignore
/// let expense_report = expense_report(&client, types::UUID::from("2bdab00d-86fb-46dc-ae05-7cc9c4aedc80")).await?;
/// ```
pub async fn expense_report(
    client: &client::Client,
    id: types::UUID,
) -> Result<expense_report::ExpenseReport, Box<dyn std::error::Error>> {
    // retrieve expense report
    Ok(client
        .read::<expense_report::ExpenseReport>(&format!("expense-reports/{id}"), None)
        .await?)
}

/// Retrieves all Ruddr Expense Report objects, and deserializes it to the corresponding vector of model structs.
/// [API Documentation](https://ruddr.readme.io/reference/list-expense-reports)
/// ```ignore
/// let expense_reports = expense_reports(&client).await?;
/// ```
pub async fn expense_reports(
    client: &client::Client,
) -> Result<expense_report::ExpenseReports, Box<dyn std::error::Error>> {
    // retrieve expense reports
    Ok(client
        .read::<expense_report::ExpenseReports>("expense_reports", Some(&"limit=100"))
        .await?)
}

#[cfg(test)]
mod tests;
