//! # Util
//!
//! `interface::util` consists of helper functions for use with the interface modules' functions.
use log;
use regex::Regex;

pub(crate) fn validate_date(date: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let date_validator = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    if date_validator.is_match(date) {
        Ok(date)
    } else {
        log::error!("{date} is an invalid date format");
        Err("invalid date")?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_date() {
        assert_eq!(
            validate_date("2024-01-01").expect("date validation errored unexpectedly"),
            "2024-01-01",
            "expected date string not returned",
        )
    }

    #[test]
    fn test_validate_date_err() {
        assert_eq!(
            validate_date("2024-01-0").unwrap_err().to_string(),
            "invalid date",
            "invalid date did not return expected error",
        )
    }
}
