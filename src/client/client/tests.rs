use super::*;
use crate::model::project;

#[tokio::test]
async fn test_client_new() {
    let client = Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    println!("client: {:?}", client)
}

#[tokio::test]
async fn test_client_new_env() {
    // unsafe because tests are multi-threaded
    unsafe {
        std::env::set_var("RUDDR_TOKEN", "abcdefghi123456789");
    }
    let client = Client::new(None).expect("client with env token could not be constructed");
    println!("client: {:?}", client);

    // unsafe because tests are multi-threaded
    unsafe {
        std::env::remove_var("RUDDR_TOKEN");
    }
    assert_eq!(
        Client::new(None).unwrap_err().to_string(),
        "ruddr api token was not input through code or RUDDR_TOKEN environment variable",
        "attempted client build without token did not error expectedly",
    )
}

#[tokio::test]
async fn test_client_read() {
    let client = Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    let response = client
        .read::<project::Project>("projects/095e0780-48bf-472c-8deb-2fc3ebc7d90c", None)
        .await
        .expect_err("read did not fail on auth");
    assert_eq!(response.status(), Some(reqwest::StatusCode::UNAUTHORIZED));
}

#[tokio::test]
async fn test_client_list() {
    let client = Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    let response = client
        .read::<project::Projects>("projects", Some("limit=100"))
        .await
        .expect_err("list did not fail on auth");
    assert_eq!(response.status(), Some(reqwest::StatusCode::UNAUTHORIZED));
}
