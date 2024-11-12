//! # Types
//!
//! `model::types` defines custom type structs with trait implementations for Ruddr API custom types.
use serde::Deserializer;

/// Custom type for Ruddr Date type
#[derive(PartialEq, Eq, Debug)]
pub struct Date(String);

impl From<String> for Date {
    fn from(date: String) -> Self {
        Date(date)
    }
}

impl From<&str> for Date {
    fn from(date: &str) -> Self {
        Date(String::from(date))
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

impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D: Deserializer<'de>>(date: D) -> Result<Self, D::Error> {
        Ok(Date(String::deserialize(date)?))
    }
}

/// Custom type for Ruddr UUID type
#[derive(PartialEq, Eq, Debug)]
pub struct UUID(String);

impl From<String> for UUID {
    fn from(uuid: String) -> Self {
        UUID(uuid)
    }
}

impl From<&str> for UUID {
    fn from(uuid: &str) -> Self {
        UUID(String::from(uuid))
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

impl<'de> serde::Deserialize<'de> for UUID {
    fn deserialize<D: Deserializer<'de>>(uuid: D) -> Result<Self, D::Error> {
        Ok(UUID(String::deserialize(uuid)?))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_date_from_str() {
        assert_eq!(Date(String::from("1234-56-78")), Date::from("1234-56-78"),)
    }

    #[test]
    fn test_date_from_string() {
        assert_eq!(
            Date::from(String::from("1234-56-78")),
            Date(String::from("1234-56-78")),
        )
    }

    #[test]
    fn test_date_to_string() {
        assert_eq!(
            String::from("1234-56-78"),
            String::from(Date(String::from("1234-56-78")))
        )
    }

    #[test]
    fn test_date_to_str() {
        assert_eq!(
            "1234-56-78",
            &String::from(Date(String::from("1234-56-78")))
        )
    }

    #[test]
    fn test_date_deserialize() {
        assert_eq!(
            Date(String::from("1234-56-78")),
            serde_json::from_str::<Date>("\"1234-56-78\"").expect("date could not be deserialized")
        )
    }

    #[test]
    fn test_uuid_from_str() {
        assert_eq!(
            UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")),
            UUID::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2"),
        )
    }

    #[test]
    fn test_uuid_from_string() {
        assert_eq!(
            UUID::from(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")),
            UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")),
        )
    }

    #[test]
    fn test_uuid_to_string() {
        assert_eq!(
            String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2"),
            String::from(UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")))
        )
    }

    #[test]
    fn test_uuid_to_str() {
        assert_eq!(
            "4c8d3f42-6efd-4a7e-85ca-d43164db0ab2",
            &String::from(UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")))
        )
    }

    #[test]
    fn test_uuid_deserialize() {
        assert_eq!(
            UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")),
            serde_json::from_str::<UUID>("\"4c8d3f42-6efd-4a7e-85ca-d43164db0ab2\"")
                .expect("uuid could not be deserialized")
        )
    }
}
