use super::*;

#[test]
fn test_time_entry() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            time_entry(
                &client,
                types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
                    .expect("invalid UUID")
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "time entry retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_time_entries() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            time_entries(
                &client,
                Some(
                    types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
                        .expect("invalid UUID")
                ),
                Some(
                    types::UUID::try_from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")
                        .expect("invalid UUID")
                ),
                Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
                Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
                Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "time entries retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
