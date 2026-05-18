use super::*;

#[tokio::test]
async fn test_customer() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        customer(
            &client,
            model::types::UUID::try_from("4cacdf11-71d1-4fbb-90ee-b091803581b0")
                .expect("uuid conversion failed")
        )
        .await
        .expect_err("client retrieval did not fail on auth")
        .status(),
        Some(reqwest::StatusCode::UNAUTHORIZED),
    )
}

#[tokio::test]
async fn test_clients() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        clients(&client, Some("JOE"))
            .await
            .expect_err("clients retrieval did not fail on auth")
            .status(),
        Some(reqwest::StatusCode::UNAUTHORIZED),
    )
}
