//! # Error
//!
//! `client::error` defines the error type returned by client construction and requests against the Ruddr API.
use reqwest::StatusCode;
use std::{error, fmt};

/// Aggregation of error types for client construction and API requests.
#[derive(Debug)]
pub enum ClientError {
    /// The API token was provided through neither the constructor parameter nor the `RUDDR_TOKEN` environment variable.
    MissingToken,
    /// The API token contains characters that are invalid in an HTTP header value.
    InvalidToken(reqwest::header::InvalidHeaderValue),
    /// The underlying HTTP client could not be built.
    Build(reqwest::Error),
    /// The request could not be completed or its response could not be read or deserialized (e.g. connection failure, timeout, decode failure).
    Transport(reqwest::Error),
    /// The API responded with a non-success status. The body is typically the API validation or error message.
    Status { status: StatusCode, body: String },
}

impl ClientError {
    /// HTTP status code associated with the error, if the failure occurred at or after receiving a response.
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            ClientError::Status { status, .. } => Some(*status),
            ClientError::Transport(error) | ClientError::Build(error) => error.status(),
            ClientError::MissingToken | ClientError::InvalidToken(_) => None,
        }
    }

    /// Response body of an API error response, if the error is a non-success status with a non-empty body.
    pub fn body(&self) -> Option<&str> {
        match self {
            ClientError::Status { body, .. } if !body.is_empty() => Some(body),
            _ => None,
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::MissingToken => write!(
                format,
                "ruddr api token was not input through code or RUDDR_TOKEN environment variable"
            ),
            ClientError::InvalidToken(error) => write!(format, "invalid ruddr api token: {error}"),
            ClientError::Build(error) => write!(format, "client could not be built: {error}"),
            ClientError::Transport(error) => write!(format, "request failed: {error}"),
            ClientError::Status { status, body } if body.is_empty() => {
                write!(format, "ruddr api responded with status {status}")
            }
            ClientError::Status { status, body } => {
                write!(format, "ruddr api responded with status {status}: {body}")
            }
        }
    }
}

impl error::Error for ClientError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ClientError::InvalidToken(error) => Some(error),
            ClientError::Build(error) | ClientError::Transport(error) => Some(error),
            ClientError::MissingToken | ClientError::Status { .. } => None,
        }
    }
}

// request execution and response reading/deser are the only contexts where `?` converts a bare `reqwest::Error`; the url is stripped because it contains the query parameters
impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        ClientError::Transport(error.without_url())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for ClientError {
    fn from(error: reqwest::header::InvalidHeaderValue) -> Self {
        ClientError::InvalidToken(error)
    }
}

#[cfg(test)]
mod tests;
