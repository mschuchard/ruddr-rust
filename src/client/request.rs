//! # Request
//!
//! `client::request` consists of functions for constructing and executing requests against the Ruddr API.
use log;

// request struct for composing request structures
#[derive(Debug)]
pub(super) struct Request {
    pub(super) url: String,
}

impl Request {
    // request constructor with endpoint and params
    pub(super) fn new(endpoint: &str, params: &str) -> Self {
        log::debug!("request endpoint is {endpoint} and params is {params}");

        Self {
            url: format!("https://www.ruddr.io/api/workspace/{endpoint}{params}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_new() {
        assert_eq!(
            Request::new("endpoint", "?params").url,
            String::from("https://www.ruddr.io/api/workspace/endpoint?params")
        )
    }
}
