use super::*;

#[test]
fn test_request_new_params() {
    assert_eq!(
        Request::new("endpoint", "params")
            .expect("request could not be constructed")
            .url,
        String::from("https://www.ruddr.io/api/workspace/endpoint?params")
    )
}

#[test]
fn test_request_new_empty_params() {
    assert_eq!(
        Request::new("endpoint", "")
            .expect("request could not be constructed")
            .url,
        String::from("https://www.ruddr.io/api/workspace/endpoint")
    )
}

#[test]
fn test_request_new_error() {
    assert_eq!(
        Request::new("", "params").unwrap_err().to_string(),
        "invalid empty endpoint"
    )
}

#[test]
fn test_request_get() {
    let test = async {
        let client = reqwest::Client::builder()
            .build()
            .expect("client with env token could not be constructed");
        let request = Request::new("projects/095e0780-48bf-472c-8deb-2fc3ebc7d90c", "")
            .expect("request could not be constructed");
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
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
