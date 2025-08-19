use super::*;

#[test]
fn test_expense_report_deserialize() {
    let json_input = r#"
        {
          "id": "2bdab00d-86fb-46dc-ae05-7cc9c4aedc80",
          "number": 1000,
          "title": "Las Vegas Convention",
          "notes": "All of the expenses for my trip to Las Vegas in March.",
          "date": "2022-03-11",
          "createdAt": "2022-03-18T15:49:07.486Z",
          "member": {
            "id": "ec5543de-3b0f-47a0-b8ef-a6e18dc4b885",
            "name": "John Smith"
          }
        }"#;
    let expense_report_deserialized = serde_json::from_str::<ExpenseReport>(json_input)
        .expect("expense_report could not be deserialized");
    let expense_report = ExpenseReport {
        id: types::UUID(String::from("2bdab00d-86fb-46dc-ae05-7cc9c4aedc80")),
        number: 1000,
        title: String::from("Las Vegas Convention"),
        notes: String::from("All of the expenses for my trip to Las Vegas in March."),
        date: types::Date(String::from("2022-03-11")),
        created_at: types::Timestamp(String::from("2022-03-18T15:49:07.486Z")),
        member: shared::Member {
            id: types::UUID(String::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
            name: String::from("John Smith"),
        },
    };
    assert_eq!(
        expense_report_deserialized, expense_report,
        "expense_report did not contain the expected values"
    );

    let json_inputs = &format!(
        r#"
        {{"results": [{json_input}],"hasMore": false}}
        "#
    );
    let expense_reports_deserialized = serde_json::from_str::<ExpenseReports>(json_inputs)
        .expect("expense reports could not be deserialized");
    let expense_reports = ExpenseReports {
        results: vec![expense_report],
        has_more: false,
    };
    assert_eq!(
        expense_reports_deserialized, expense_reports,
        "expense_reports did not contain the expected values"
    );
}
