use super::*;

#[test]
fn test_role() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            role(
                &client,
                types::UUID::try_from("7ad5a34a-07b7-48e9-a760-bd220d52e354")
                    .expect("uuid conversion failed")
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "role retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_roles() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            roles(
                &client,
                Some(
                    types::UUID::try_from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")
                        .expect("uuid conversion failed")
                ),
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "roles retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
