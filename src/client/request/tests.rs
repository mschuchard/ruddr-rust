use super::*;

#[test]
fn test_request_new_params() {
    assert_eq!(
        Request::new("endpoint", Some("params")).url,
        Url::parse("https://www.ruddr.io/api/workspace/endpoint?params")
            .expect("failed to parse URL")
    )
}

#[test]
fn test_request_new_empty_params() {
    assert_eq!(
        Request::new("endpoint", None).url,
        Url::parse("https://www.ruddr.io/api/workspace/endpoint").expect("failed to parse URL")
    )
}

#[test]
#[should_panic(expected = "endpoint must not be empty")]
fn test_request_new_empty_endpoint_error() {
    Request::new("", Some("params"));
}

#[tokio::test]
async fn test_request_get() {
    let client = reqwest::Client::builder()
        .build()
        .expect("client with env token could not be constructed");
    let request = Request::new("projects/095e0780-48bf-472c-8deb-2fc3ebc7d90c", None);
    let response = request
        .get(&client)
        .await
        .expect("request transmission failed to receive a response");
    println!("response: {:?}", response);
    assert_eq!(
        response.status(),
        401,
        "the response did not return expected 401 status",
    )
}
