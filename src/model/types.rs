//! # Types
//!
//! `model::types` defines custom type structs with trait implementations corresponding to Ruddr API custom types.
use regex::Regex;
use serde::Deserialize;
use std::fmt;

/// Custom type for Ruddr Date type in YYYY-MM-DD format.
/// Consumers are expected to instantiate this through type conversion, and not the implicit or explicit constructors.
/// ```ignore
/// Date::from("2028-12-31")
/// ```
#[derive(PartialEq, Eq, Deserialize, Debug)]
// public access to the type should exist, but not to the implicit constructor as users are expected to instantiate through type converters each containing an invocation to the explicit constructor
pub struct Date(pub(super) String);

impl Date {
    // constructor with validation used within type converters
    fn new(date: String) -> Result<Self, String> {
        let date_validator = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        if date_validator.is_match(&date) {
            Ok(Date(date))
        } else {
            Err(format!("invalid date: {date}"))
        }
    }
}

impl TryFrom<String> for Date {
    type Error = String;

    fn try_from(date: String) -> Result<Self, Self::Error> {
        Date::new(date)
    }
}

impl TryFrom<&str> for Date {
    type Error = String;

    fn try_from(date: &str) -> Result<Self, Self::Error> {
        Date::new(String::from(date))
    }
}

impl From<Date> for String {
    fn from(date: Date) -> Self {
        date.0
    }
}

impl<'date> From<&'date Date> for &'date str {
    fn from(date: &'date Date) -> Self {
        &date.0
    }
}

/*impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Date::new(String::deserialize(deserializer)?))
    }
}

impl serde::Serialize for Date {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}*/

impl fmt::Display for Date {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "{}", self.0)
    }
}

/// Custom type for Ruddr Timestamp type in YYYY-MM-DDThh:mm:ss.msZ format where "T" is literal.
/// This is most similar to ISO 8601 extended format with milliseconds for reference.
/// Consumers are expected to instantiate this through type conversion, and not the implicit or explicit constructors.
/// ```ignore
/// Timestamp::from("1234-56-78T12:34:56.789Z")
/// ```
/// This type is currently unused in input parameters to interface functions, but is still public for potential unforeseen usage.
#[derive(PartialEq, Eq, Deserialize, Debug)]
// public access to the type should exist, but not to the implicit constructor as users are expected to instantiate through type converters each containing an invocation to the explicit constructor
pub struct Timestamp(pub(super) String);

impl Timestamp {
    // constructor with validation used within type converters
    fn new(timestamp: String) -> Result<Self, String> {
        let timestamp_validator =
            Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$").unwrap();
        if timestamp_validator.is_match(&timestamp) {
            Ok(Timestamp(timestamp))
        } else {
            Err(format!("invalid timestamp: {timestamp}"))
        }
    }
}

impl TryFrom<String> for Timestamp {
    type Error = String;

    fn try_from(timestamp: String) -> Result<Self, Self::Error> {
        Timestamp::new(timestamp)
    }
}

impl TryFrom<&str> for Timestamp {
    type Error = String;

    fn try_from(timestamp: &str) -> Result<Self, Self::Error> {
        Timestamp::new(String::from(timestamp))
    }
}

impl From<Timestamp> for String {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}

impl<'timestamp> From<&'timestamp Timestamp> for &'timestamp str {
    fn from(timestamp: &'timestamp Timestamp) -> Self {
        &timestamp.0
    }
}

/*impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Timestamp::new(String::deserialize(deserializer)?))
    }
}

impl serde::Serialize for Timestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}*/

impl fmt::Display for Timestamp {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "{}", self.0)
    }
}

/// Custom type for Ruddr UUID type in standard format.
/// Consumers are expected to instantiate this through type conversion, and not the implicit or explicit constructors.
/// ```ignore
/// UUID::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")
/// ```
#[derive(PartialEq, Eq, Deserialize, Debug)]
// public access to the type should exist, but not to the implicit constructor as users are expected to instantiate through type converters each containing an invocation to the explicit constructor
pub struct UUID(pub(super) String);

impl UUID {
    // constructor with validation used within type converters
    fn new(uuid: String) -> Result<Self, String> {
        let uuid_validator = Regex::new(r"^\w{8}-\w{4}-\w{4}-\w{4}-\w{12}$").unwrap();
        if uuid_validator.is_match(&uuid) {
            Ok(UUID(uuid))
        } else {
            Err(format!("invalid uuid: {uuid}"))
        }
    }
}

impl TryFrom<String> for UUID {
    type Error = String;

    fn try_from(uuid: String) -> Result<Self, Self::Error> {
        UUID::new(uuid)
    }
}

impl TryFrom<&str> for UUID {
    type Error = String;

    fn try_from(uuid: &str) -> Result<Self, Self::Error> {
        UUID::new(String::from(uuid))
    }
}

impl From<UUID> for String {
    fn from(uuid: UUID) -> Self {
        uuid.0
    }
}

impl<'uuid> From<&'uuid UUID> for &'uuid str {
    fn from(uuid: &'uuid UUID) -> Self {
        &uuid.0
    }
}

/*impl<'de> serde::Deserialize<'de> for UUID {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(UUID::new(String::deserialize(deserializer)?))
    }
}

impl serde::Serialize for UUID {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}*/

impl fmt::Display for UUID {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "{}", self.0)
    }
}

/// Custom type for Ruddr Slug type in standard format.
/// Consumers are expected to instantiate this through type conversion, and not the implicit or explicit constructors.
/// ```ignore
/// Slug::from("vendor-portal")
/// ```
#[derive(PartialEq, Eq, Deserialize, Debug)]
// public access to the type should exist, but not to the implicit constructor as users are expected to instantiate through type converters each containing an invocation to the explicit constructor
pub struct Slug(pub(super) String);

impl Slug {
    // constructor with validation used within type converters
    fn new(slug: String) -> Result<Self, String> {
        let slug_validator = Regex::new(r"^[a-z0-9-]+$").unwrap();
        if slug_validator.is_match(&slug) {
            Ok(Slug(slug))
        } else {
            Err(format!("invalid slug: {slug}"))
        }
    }
}

impl TryFrom<String> for Slug {
    type Error = String;

    fn try_from(slug: String) -> Result<Self, Self::Error> {
        Slug::new(slug)
    }
}

impl TryFrom<&str> for Slug {
    type Error = String;

    fn try_from(slug: &str) -> Result<Self, Self::Error> {
        Slug::new(String::from(slug))
    }
}

impl From<Slug> for String {
    fn from(slug: Slug) -> Self {
        slug.0
    }
}

impl<'slug> From<&'slug Slug> for &'slug str {
    fn from(slug: &'slug Slug) -> Self {
        &slug.0
    }
}

/*impl<'de> serde::Deserialize<'de> for Slug {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Slug::new(String::deserialize(deserializer)?))
    }
}

impl serde::Serialize for Slug {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}*/

impl fmt::Display for Slug {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "{}", self.0)
    }
}

#[cfg(test)]
mod tests;
