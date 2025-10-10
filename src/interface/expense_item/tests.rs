use super::*;

#[test]
fn test_expense_item() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            expense_item(
                &client,
                types::UUID::try_from("77f5ccdc-4226-4ff1-877e-5644d0a04522")
                    .expect("uuid conversion failed")
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "expense_item retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_expense_items() {
    let test = async {
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
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "expense_items retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
