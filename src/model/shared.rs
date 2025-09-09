//! # Shared
//!
//! This module contains model structs shared across the Ruddr API objects.
use crate::model::types;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: types::UUID,
    pub name: String,
    pub client: Client,
}

pub type Member = Entity;
pub type Client = Entity;

// Simple generic entity struct for models comprised of only an ID and a name.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: types::UUID,
    pub name: String,
}
