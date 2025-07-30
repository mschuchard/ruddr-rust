use super::*;

#[test]
fn test_cost() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            cost(
                &client,
                types::UUID::from("b3a100b0-8e71-4f39-9d96-32f11838aa8c")
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "cost retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_costs() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            costs(
                &client,
                Some(types::UUID::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "costs retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
