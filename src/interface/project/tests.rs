use super::*;

#[test]
fn test_project() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            project(
                &client,
                types::UUID::try_from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")
                    .expect("invalid UUID")
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "project retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_projects() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            projects(
                &client,
                Some(
                    types::UUID::try_from("d5afaffe-09e5-4d73-b02c-905b40fc6c22")
                        .expect("invalid UUID")
                ),
                Some(
                    types::UUID::try_from("9b0927a6-35a1-4795-a4ca-10167b05f7de")
                        .expect("invalid UUID")
                ),
                Some(enums::Status::InProgress),
                Some("my_project"),
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "projects retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
