//! # Customer
//!
//! `model::customer` is a model for the Ruddr Client object. This module is not publically accessible, but the structs and members are public for reading from `interface::customer` returns.
//! [API Documentation](https://ruddr.readme.io/reference/client-object)
use crate::model::{shared, types};
use serde::Deserialize;

/// Model for Clients used with List operations.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clients {
    pub results: Vec<Client>,
    pub has_more: bool,
}

/// Model for Client used with Read operations.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub id: types::UUID,
    pub key: String,
    pub name: String,
    pub code: String,
    pub currency: String,
    pub notes: String,
    pub emails: Vec<String>,
    pub street_address: String,
    pub invoice_details_source: InvoiceDetailsSource,
    pub invoice_subject: String,
    pub invoice_email_subject: String,
    pub invoice_email_body: String,
    pub use_workspace_invoice_details: bool,
    pub invoice_notes: String,
    pub is_internal: bool,
    pub record_status_id: RecordStatus,
    pub created_at: types::Timestamp,
    pub practice: Option<shared::Entity>,
    pub invoice_payment_term: shared::Entity,
    pub owner: shared::Entity,
    pub tags: Vec<shared::Entity>,
    pub business_unit: Option<shared::Entity>,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceDetailsSource {
    Workspace,
    Custom,
    BusinessUnit,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RecordStatus {
    Active,
    Archived,
}

#[cfg(test)]
mod tests;
