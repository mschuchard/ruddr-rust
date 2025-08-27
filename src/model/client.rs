//! # Customer
//!
//! `model::customer` is a model for the Ruddr Client object. This module is not publically accessible, but the structs and members are public for reading from `interface::customer` returns.
//! [API Documentation](https://ruddr.readme.io/reference/client-object)
use crate::model::types;
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
    pub practice: Option<Practice>,
    pub invoice_payment_term: InvoicePaymentTerm,
    pub owner: Owner,
    pub tags: Vec<Tag>,
    pub business_unit: Option<BusinessUnit>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Practice {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoicePaymentTerm {
    pub id: types::UUID,
    pub name: PaymentTerms,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessUnit {
    pub id: types::UUID,
    pub name: String,
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
pub enum PaymentTerms {
    #[serde(rename = "due_on_receipt")]
    DueOnReceipt,
    #[serde(rename = "Net-10")]
    Net10,
    #[serde(rename = "Net-15")]
    Net15,
    #[serde(rename = "Net-30")]
    Net30,
    #[serde(rename = "Net-45")]
    Net45,
    #[serde(rename = "Net-60")]
    Net60,
    #[serde(rename = "Net-90")]
    Net90,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RecordStatus {
    Active,
    Archived,
}

#[cfg(test)]
mod tests;
