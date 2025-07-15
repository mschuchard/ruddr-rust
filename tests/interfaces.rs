use ruddr::client::client;
use ruddr::interface::*;
use ruddr::model::{enums, types};

#[test]
fn test_allocations() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            allocation::allocations(
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

#[test]
fn test_customers() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            customer::customers(&client, Some("JOE"))
                .await
                .unwrap_err()
                .to_string(),
            "client read response failed",
            "clients retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_expense_items() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            expense_item::expense_items(
                &client,
                Some(types::UUID::from("2bdab00d-86fb-46dc-ae05-7cc9c4aedc80"))
            )
            .await
            .unwrap_err()
            .to_string(),
            "client read response failed",
            "expense_items retrieval did not fail on auth",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_expense_reports() {
    let test = async {
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
            member::members(&client, Some("Joe"), Some("foo@bar.com"))
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

#[test]
fn test_projects() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            project::projects(
                &client,
                Some(types::UUID::from("d5afaffe-09e5-4d73-b02c-905b40fc6c22")),
                Some(types::UUID::from("9b0927a6-35a1-4795-a4ca-10167b05f7de")),
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

#[test]
fn test_time_entries() {
    let test = async {
        let client = client::Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        assert_eq!(
            time::time_entries(
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
