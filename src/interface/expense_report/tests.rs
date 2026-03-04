use super::*;

#[tokio::test]
async fn test_expense_report() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        expense_report(
            &client,
            types::UUID::try_from("2bdab00d-86fb-46dc-ae05-7cc9c4aedc80")
                .expect("uuid conversion failed")
        )
        .await
        .unwrap_err()
        .to_string(),
        "client read response failed",
        "expense_report retrieval did not fail on auth",
    )
}

#[tokio::test]
async fn test_expense_reports() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        expense_reports(&client).await.unwrap_err().to_string(),
        "client read response failed",
        "expense_reports retrieval did not fail on auth",
    )
}
