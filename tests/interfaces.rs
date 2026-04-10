use ruddr::client::client;
use ruddr::interface::*;
use ruddr::model::{enums, types};

#[tokio::test]
async fn test_allocations() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        allocation::allocations(
            &client,
            Some(enums::AssignmentType::Project),
            Some(
                types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
                    .expect("uuid conversion failed")
            ),
            Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
            Some(types::Date::try_from("2024-01-01").expect("date conversion failed")),
        )
        .await
        .unwrap_err()
        .to_string(),
        "client read response failed",
        "allocations retrieval did not fail on auth",
    )
}

#[tokio::test]
async fn test_customers() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        customer::clients(&client, Some("JOE"))
            .await
            .unwrap_err()
            .to_string(),
        "client read response failed",
        "clients retrieval did not fail on auth",
    )
}

#[tokio::test]
async fn test_costs() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        cost::costs(
            &client,
            Some(
                types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
                    .expect("uuid conversion failed")
            ),
        )
        .await
        .unwrap_err()
        .to_string(),
        "client read response failed",
        "costs retrieval did not fail on auth",
    )
}

#[tokio::test]
async fn test_expense_items() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        expense_item::expense_items(
            &client,
            Some(
                types::UUID::try_from("2bdab00d-86fb-46dc-ae05-7cc9c4aedc80")
                    .expect("uuid conversion failed")
            )
        )
        .await
        .unwrap_err()
        .to_string(),
        "client read response failed",
        "expense_items retrieval did not fail on auth",
    )
}

#[tokio::test]
async fn test_expense_reports() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        expense_report::expense_reports(&client)
            .await
            .unwrap_err()
            .to_string(),
        "client read response failed",
        "expense_reports retrieval did not fail on auth",
    )
}

#[tokio::test]
async fn test_members() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        member::members(&client, Some("Joe"), Some("foo@bar.com"))
            .await
            .unwrap_err()
            .to_string(),
        "client read response failed",
        "members retrieval did not fail on auth",
    )
}

#[tokio::test]
async fn test_projects() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        project::projects(
            &client,
            Some(
                types::UUID::try_from("d5afaffe-09e5-4d73-b02c-905b40fc6c22")
                    .expect("uuid conversion failed")
            ),
            Some(
                types::UUID::try_from("9b0927a6-35a1-4795-a4ca-10167b05f7de")
                    .expect("uuid conversion failed")
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
}

#[tokio::test]
async fn test_roles() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        role::roles(
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
}

#[tokio::test]
async fn test_time_entries() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        time::time_entries(
            &client,
            Some(
                types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
                    .expect("uuid conversion failed")
            ),
            Some(
                types::UUID::try_from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")
                    .expect("uuid conversion failed")
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
}

#[tokio::test]
async fn test_utilizations() {
    let client = client::Client::new(Some("abcdefghi123456789"))
        .expect("client with token could not be constructed");
    assert_eq!(
        utilization::utilizations(
            &client,
            Some(
                types::UUID::try_from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")
                    .expect("uuid conversion failed")
            ),
        )
        .await
        .unwrap_err()
        .to_string(),
        "client read response failed",
        "utilizations retrieval did not fail on auth",
    )
}
