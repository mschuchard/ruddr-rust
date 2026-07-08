use super::*;

#[test]
fn test_expense_item_deserialize() {
    let json_input = r#"
        {
          "id": "77f5ccdc-4226-4ff1-877e-5644d0a04522",
          "statusId": "approved",
          "vendor": "Delta",
          "notes": "Flight to LAS",
          "attendees": "Alice Johnson, Bob Smith",
          "date": "2022-03-11",
          "currency": "USD",
          "amount": 345.36,
          "markupMethod": "amount",
          "markupRatio": null,
          "markupAmount": 5,
          "markup": 5,
          "total": 350.36,
          "unitCount": 23,
          "unitAmount": 0.575,
          "isReimbursable": true,
          "isBillable": true,
          "invoiced": false,
          "createdAt": "2022-03-18T16:51:51.148Z",
          "expenseReport": {
            "id": "2bdab00d-86fb-46dc-ae05-7cc9c4aedc80",
            "title": "Las Vegas Convention"
          },
          "expenseCategory": {
            "id": "175e0635-ac9e-4880-8492-07fa584f1b15",
            "name": "Airfare",
            "unitName": "mile"
          },
          "member": {
            "id": "ec5543de-3b0f-47a0-b8ef-a6e18dc4b885",
            "name": "John Smith"
          },
          "project": null
        }"#;
    let expense_item_deserialized = serde_json::from_str::<ExpenseItem>(json_input)
        .expect("expense_item could not be deserialized");
    let expense_item = ExpenseItem {
        id: types::UUID(String::from("77f5ccdc-4226-4ff1-877e-5644d0a04522")),
        status_id: Status::Approved,
        vendor: Some(String::from("Delta")),
        notes: Some(String::from("Flight to LAS")),
        attendees: Some(String::from("Alice Johnson, Bob Smith")),
        date: types::Date(String::from("2022-03-11")),
        currency: String::from("USD"),
        amount: 345.36,
        markup_method: Some(MarkupMethod::Amount),
        markup_ratio: None,
        markup_amount: Some(5.0),
        markup: Some(5.0),
        total: 350.36,
        unit_count: Some(23),
        unit_amount: Some(0.575),
        is_reimbursable: true,
        is_billable: true,
        invoiced: false,
        created_at: types::Timestamp(String::from("2022-03-18T16:51:51.148Z")),
        expense_report: ExpenseReport {
            id: types::UUID(String::from("2bdab00d-86fb-46dc-ae05-7cc9c4aedc80")),
            title: String::from("Las Vegas Convention"),
        },
        expense_category: ExpenseCategory {
            id: types::UUID(String::from("175e0635-ac9e-4880-8492-07fa584f1b15")),
            name: String::from("Airfare"),
            unit_name: Some(String::from("mile")),
        },
        member: shared::Entity {
            id: types::UUID(String::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
            name: String::from("John Smith"),
        },
        project: None,
    };
    assert_eq!(
        expense_item_deserialized, expense_item,
        "expense_item did not contain the expected values"
    );

    let json_inputs = &format!(
        r#"
        {{"results": [{json_input}],"hasMore": false}}
        "#
    );
    let expense_items_deserialized = serde_json::from_str::<ExpenseItems>(json_inputs)
        .expect("expense items could not be deserialized");
    let expense_items = ExpenseItems {
        results: vec![expense_item],
        has_more: false,
    };
    assert_eq!(
        expense_items_deserialized, expense_items,
        "expense_items did not contain the expected values"
    );
}
