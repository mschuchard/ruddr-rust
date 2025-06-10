use super::*;

#[test]
fn test_allocation() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            allocation(
                &client,
                types::UUID::from("212b8272-ed2a-4a91-950a-8a06b3546144")
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "allocation retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_allocations() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            allocations(
                &client,
                Some(enums::AssignmentType::Project),
                Some(types::UUID::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
                Some(types::Date::from("2024-01-01")),
                Some(types::Date::from("2024-01-01")),
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "allocations retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
