use super::*;

#[test]
fn test_client_error_missing_token() {
    let error = ClientError::MissingToken;
    assert_eq!(
        error.to_string(),
        "ruddr api token was not input through code or RUDDR_TOKEN environment variable"
    );
    assert_eq!(error.status(), None);
    assert_eq!(error.body(), None);
}

#[test]
fn test_client_error_status_with_body() {
    let error = ClientError::Status {
        status: StatusCode::BAD_REQUEST,
        body: String::from("{\"message\":\"name is required\"}"),
    };
    assert_eq!(
        error.to_string(),
        "ruddr api responded with status 400 Bad Request: {\"message\":\"name is required\"}"
    );
    assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
    assert_eq!(error.body(), Some("{\"message\":\"name is required\"}"));
}

#[test]
fn test_client_error_status_empty_body() {
    let error = ClientError::Status {
        status: StatusCode::UNAUTHORIZED,
        body: String::new(),
    };
    assert_eq!(
        error.to_string(),
        "ruddr api responded with status 401 Unauthorized"
    );
    assert_eq!(error.status(), Some(StatusCode::UNAUTHORIZED));
    assert_eq!(error.body(), None);
}
