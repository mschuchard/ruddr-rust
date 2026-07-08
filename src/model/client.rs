//! # Client
//!
//! `model::client` is a model for the Ruddr Client object. This module is not publically accessible, but the structs and members are public for reading from `interface::client` returns.
//! [API Documentation](https://docs.ruddr.io/api-reference/clients/get-a-client.md)
use crate::model::{shared, types};
use serde::{Deserialize, Serialize};

/// Model for Clients used with List operations.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Clients {
    pub results: Vec<Client>,
    pub has_more: bool,
}

/// Model for Client used with Read operations.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub id: types::UUID,
    pub key: String,
    pub name: String,
    pub code: Option<String>,
    pub currency: String,
    pub notes: Option<String>,
    pub emails: Vec<String>,
    pub street_address: Option<String>,
    pub invoice_details_source: InvoiceDetailsSource,
    pub invoice_subject: Option<String>,
    pub invoice_email_subject: Option<String>,
    pub invoice_email_body: Option<String>,
    pub use_workspace_invoice_details: bool,
    pub invoice_notes: Option<String>,
    pub is_internal: bool,
    pub record_status_id: RecordStatus,
    pub created_at: types::Timestamp,
    pub practice: Option<shared::Entity>,
    pub industry: Option<shared::Entity>,
    pub location: Option<shared::Entity>,
    pub invoice_payment_term: Option<shared::Entity>,
    pub owner: Option<shared::Entity>,
    pub tags: Vec<shared::Entity>,
    pub sales_representative: Option<shared::Entity>,
    pub business_unit: Option<shared::Entity>,
    pub integrations: Vec<Integration>,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Integration {
    #[serde(rename = "type")]
    pub integration_type: IntegrationType,
    pub connection_id: types::UUID,
    pub external_id: String,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceDetailsSource {
    Workspace,
    Custom,
    BusinessUnit,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RecordStatus {
    Active,
    Archived,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationType {
    Xero,
    Qbo,
}

#[cfg(test)]
mod tests;
