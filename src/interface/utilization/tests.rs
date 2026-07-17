use super::*;

#[tokio::test]
async fn test_utilization() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        utilization(
            &client,
            types::UUID::try_from("8e6d6316-5bc2-4135-b99c-f604f29051ab").expect("invalid UUID")
        )
        .await
        .expect_err("utilization retrieval did not fail on auth")
        .status(),
        Some(reqwest::StatusCode::UNAUTHORIZED),
    )
}

#[tokio::test]
async fn test_utilizations() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        utilizations(
            &client,
            Some(
                types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
                    .expect("invalid UUID")
            ),
            None,
            None,
        )
        .await
        .expect_err("utilizations retrieval did not fail on auth")
        .status(),
        Some(reqwest::StatusCode::UNAUTHORIZED),
    )
}
