use super::*;

#[tokio::test]
async fn test_allocation() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        allocation(
            &client,
            types::UUID::try_from("212b8272-ed2a-4a91-950a-8a06b3546144")
                .expect("uuid conversion failed")
        )
        .await
        .expect_err("allocation retrieval did not fail on auth")
        .status(),
        Some(reqwest::StatusCode::UNAUTHORIZED),
    )
}

#[tokio::test]
async fn test_allocations() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        allocations(
            &client,
            None,
            None,
            Some(enums::AssignmentType::Project),
            Some(
                types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
                    .expect("invalid UUID")
            ),
            Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
            Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
        )
        .await
        .expect_err("allocations retrieval did not fail on auth")
        .status(),
        Some(reqwest::StatusCode::UNAUTHORIZED),
    )
}
