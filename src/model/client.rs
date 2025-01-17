//! # Client
//!
//! `model::client` is a model for the Ruddr Client object
use crate::model::types;
use log::Record;
use serde::Deserialize;

/// Model for Clients used with List operations
#[derive(Debug, PartialEq, Deserialize)]
pub struct Clients {
    pub(crate) results: Vec<Client>,
}

/// Model for Client used with Read operations
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub(crate) id: types::UUID,
    pub(crate) key: String, // slug?
    pub(crate) name: String,
    pub(crate) code: String,
    pub(crate) currency: String,
    pub(crate) notes: String,
    pub(crate) emails: Vec<String>,
    pub(crate) street_address: String,
    pub(crate) use_workspace_invoice_details: bool,
    pub(crate) payment_terms_id: PaymentTerms,
    pub(crate) invoice_notes: String,
    pub(crate) is_internal: bool,
    pub(crate) record_status_id: RecordStatus,
    pub(crate) created_at: types::Timestamp,
    pub(crate) practice: Option<Practice>,
    pub(crate) owner: Owner,
    pub(crate) tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Practice {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Owner {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Tag {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PaymentTerms {
    DueOnReceipt,
    Net10,
    Net15,
    Net30,
    Net45,
    Net60,
    Net90,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RecordStatus {
    Active,
    Archived,
}
