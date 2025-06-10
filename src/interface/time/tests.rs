use super::*;

#[test]
fn test_time_entry() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            time_entry(
                &client,
                types::UUID::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "time entry retrieval did not fail on json decoding",
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
                Some(types::UUID::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
                Some(types::UUID::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")),
                Some(types::Date::from("2024-01-01")),
                Some(types::Date::from("2024-01-01")),
                Some(types::Date::from("2024-01-01")),
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
