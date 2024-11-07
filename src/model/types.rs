//! # Types
//!
//! `model::types` defines custom type structs with trait implementations for Ruddr API custom types.
use serde::Deserializer;

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

mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Date(String::from("1234-56-78")), Date::from("1234-56-78"),)
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            Date::from(String::from("1234-56-78")),
            Date(String::from("1234-56-78")),
        )
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            String::from("1234-56-78"),
            String::from(Date(String::from("1234-56-78")))
        )
    }

    #[test]
    fn test_to_str() {
        assert_eq!(
            "1234-56-78",
            &String::from(Date(String::from("1234-56-78")))
        )
    }
}
