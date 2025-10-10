use super::*;

#[test]
fn test_utilization() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            utilization(
                &client,
                types::UUID::try_from("8e6d6316-5bc2-4135-b99c-f604f29051ab")
                    .expect("invalid UUID")
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "utilization retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_utilizations() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            utilizations(
                &client,
                Some(
                    types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
                        .expect("invalid UUID")
                ),
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "utilizations retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
