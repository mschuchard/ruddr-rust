//! # Reader
//!
//! `model::reader` is a generic public function for crate consumers to access the serialized model structs. This enables member attribute reading without access to the structs themselves, and enables consumption of `interface::*::*` returns in a standardized format.

use crate::model::allocation;

pub fn allocation_read(allocation: allocation::Allocation) -> Result<String, serde_json::Error> {
    return serde_json::to_string::<allocation::Allocation>(&allocation);
}
