use regex::Regex;
use log;

pub(crate) fn validate_date(date: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let date_validator = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    if date_validator.is_match(date) {
        Ok(date)
    } else {
        log::error!("{date} is an invalid date format");
        Err("invalid date")?
    }
}

pub(crate) fn validate_uuid(uuid: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let uuid_validator = Regex::new(r"\w{8}-\w{4}-\w{4}-\w{4}-\w{12}").unwrap();
    if uuid_validator.is_match(uuid) {
        Ok(uuid)
    } else {
        log::error!("{uuid} is an invalid uuid format");
        Err("invalid uuid")?
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

    #[test]
    fn test_validate_uuid() {
        assert_eq!(
            validate_uuid("095e0780-48bf-472c-8deb-2fc3ebc7d90c").expect("uuid validation errored unexpectedly"),
            "095e0780-48bf-472c-8deb-2fc3ebc7d90c",
            "expected uuid string not returned",
        )
    }

    #[test]
    fn test_validate_uuid_err() {
        assert_eq!(
            validate_uuid("095e0780-48bf-472c-8deb-2fc3ebc7d90").unwrap_err().to_string(),
            "invalid uuid",
            "invalid uuid did not return expected error",
        )
    }
}
