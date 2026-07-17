use super::*;

#[tokio::test]
async fn test_member() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        member(
            &client,
            types::UUID::try_from("3f3df320-dd95-4a42-8eae-99243fb2ea86").expect("invalid UUID")
        )
        .await
        .expect_err("member retrieval did not fail on auth")
        .status(),
        Some(reqwest::StatusCode::UNAUTHORIZED),
    )
}

#[tokio::test]
async fn test_members() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        members(&client, Some("Joe"), Some("foo@bar.com"), None, None)
            .await
            .expect_err("members retrieval did not fail on auth")
            .status(),
        Some(reqwest::StatusCode::UNAUTHORIZED),
    )
}
