use super::*;

#[tokio::test]
async fn test_expense_item() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        expense_item(
            &client,
            types::UUID::try_from("77f5ccdc-4226-4ff1-877e-5644d0a04522")
                .expect("uuid conversion failed")
        )
        .await
        .expect_err("expense_item retrieval did not fail on auth")
        .status(),
        Some(reqwest::StatusCode::UNAUTHORIZED),
    )
}

#[tokio::test]
async fn test_expense_items() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        expense_items(
            &client,
            Some(
                types::UUID::try_from("2bdab00d-86fb-46dc-ae05-7cc9c4aedc80")
                    .expect("invalid UUID")
            )
        )
        .await
        .expect_err("expense_items retrieval did not fail on auth")
        .status(),
        Some(reqwest::StatusCode::UNAUTHORIZED),
    )
}
