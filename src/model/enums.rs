//! # Enums
//!
//! `model::enums` consists of `enum` for various Ruddr model types that should be publically "usable" to external consumers of this crate. This is typically because the types are used as inputs to interface functions.
use serde::{Deserialize, Serialize};
use std::fmt;

/// Allocation: Assignment Type
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AssignmentType {
    Project,
    TimeOff,
}

impl fmt::Display for AssignmentType {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        // use serialize for automatic snake case from trait derivation, but then remove extraneous " chars incurred during JSON formatting
        write!(
            format,
            "{}",
            serde_json::to_string(self).unwrap().replace("\"", "")
        )
    }
}

/// Project: Project Status
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Tentative,
    NotStarted,
    InProgress,
    Paused,
    Completed,
    Cancelled,
}

impl fmt::Display for Status {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        // use serialize for automatic snake case from trait derivation, but then remove extraneous " chars incurred during JSON formatting
        write!(
            format,
            "{}",
            serde_json::to_string(self).unwrap().replace("\"", "")
        )
    }
}

#[cfg(test)]
mod tests;
