use super::*;

#[test]
fn test_customer() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            customer(
                &client,
                model::types::UUID::from("4cacdf11-71d1-4fbb-90ee-b091803581b0")
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "client retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_clients() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            clients(&client, Some("JOE")).await.unwrap_err().to_string(),
            "client read response failed",
            "clients retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
