use super::*;

#[test]
fn test_member() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            member(
                &client,
                types::UUID::from("3f3df320-dd95-4a42-8eae-99243fb2ea86")
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "member retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_members() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            members(&client, Some("Joe"), Some("foo@bar.com"))
                .await
                .unwrap_err()
                .to_string(),
            "client read response failed",
            "members retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
