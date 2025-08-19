use super::*;

#[test]
fn test_project_deserialize() {
    let json_input = r#"
        {
          "id": "095e0780-48bf-472c-8deb-2fc3ebc7d90c",
          "key": "vendor-portal",
          "name": "Vendor Portal",
          "notes": "The client would like to develop a mobile app that rewards its customers for repeat purchases.",
          "statusId": "in_progress",
          "start": "2021-09-01",
          "end": "2022-01-31",
          "code": "P-2021-00068",
          "poNumber": "DM-2021-02059",
          "billingTypeId": "fixed_recurring",
          "isBillable": true,
          "currency": "USD",
          "revenueRecognitionMethod": "manual",
          "fixedFee": null,
          "fixedRecurringFee": null,
          "fixedRecurringStart": null,
          "fixedRecurringEnd": null,
          "useRoles": true,
          "useBudget": true,
          "budgetMode": "detailed",
          "useMonthlyBudget": true,
          "monthlyBudgetMode": "detailed",
          "requiresNotes": true,
          "requiresTasks": true,
          "recordStatusId": "active",
          "isProductive": null,
          "createdAt": "2022-03-15T14:59:18.825Z",
          "client": {
            "id": "d5afaffe-09e5-4d73-b02c-905b40fc6c22",
            "name": "Acme Company"
          },
          "practice": {
            "id": "40f95471-7f7c-4ffa-b838-8dcccab0f54a",
            "name": "Digital Transformation"
          },
          "projectType": {
            "id": "9b0927a6-35a1-4795-a4ca-10167b05f7de",
            "name": "Content Management"
          },
          "tags": [
            {
              "id": "626db436-98bf-40cb-9937-c382af5d818c",
              "name": "Atlanta Office"
            },
            {
              "id": "9f26fb15-23f1-49a6-8558-c19ad4338472",
              "name": "Data Analytics"
            }
          ],
          "budget": {
            "revenue": 602500,
            "servicesRevenue": 600000,
            "otherRevenue": 2000,
            "billableExpenses": 500,
            "nonBillableExpenses": 150,
            "billableHours": 150,
            "nonBillableHours": 10
          },
          "monthlyBudget": {
            "revenue": 51220,
            "servicesRevenue": 50000,
            "otherRevenue": 920,
            "billableExpenses": 300,
            "nonBillableExpenses": 50,
            "billableHours": 10,
            "nonBillableHours": 2
          }
        }"#;
    let project_deserialized =
        serde_json::from_str::<Project>(json_input).expect("time entry could not be deserialized");
    let project = Project {
        id: types::UUID(String::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")),
        key: types::Slug(String::from("vendor-portal")),
        name: String::from("Vendor Portal"),
        notes: String::from(
            "The client would like to develop a mobile app that rewards its customers for repeat purchases.",
        ),
        status_id: enums::Status::InProgress,
        start: Some(types::Date(String::from("2021-09-01"))),
        end: Some(types::Date(String::from("2022-01-31"))),
        code: String::from("P-2021-00068"),
        po_number: String::from("DM-2021-02059"),
        billing_type_id: BillingType::FixedRecurring,
        is_billable: true,
        currency: String::from("USD"),
        revenue_recognition_method: Some(RevenueRecognitionMethod::Manual),
        fixed_fee: None,
        fixed_recurring_fee: None,
        fixed_recurring_start: None,
        fixed_recurring_end: None,
        use_roles: true,
        use_budget: true,
        budget_mode: Some(BudgetMode::Detailed),
        use_monthly_budget: true,
        monthly_budget_mode: Some(MonthlyBudgetMode::Detailed),
        requires_notes: true,
        requires_tasks: true,
        record_status_id: RecordStatus::Active,
        is_productive: None,
        created_at: types::Timestamp(String::from("2022-03-15T14:59:18.825Z")),
        client: shared::Client {
            id: types::UUID(String::from("d5afaffe-09e5-4d73-b02c-905b40fc6c22")),
            name: String::from("Acme Company"),
        },
        practice: Practice {
            id: types::UUID(String::from("40f95471-7f7c-4ffa-b838-8dcccab0f54a")),
            name: String::from("Digital Transformation"),
        },
        project_type: ProjectType {
            id: types::UUID(String::from("9b0927a6-35a1-4795-a4ca-10167b05f7de")),
            name: String::from("Content Management"),
        },
        tags: vec![
            Tag {
                id: types::UUID(String::from("626db436-98bf-40cb-9937-c382af5d818c")),
                name: String::from("Atlanta Office"),
            },
            Tag {
                id: types::UUID(String::from("9f26fb15-23f1-49a6-8558-c19ad4338472")),
                name: String::from("Data Analytics"),
            },
        ],
        budget: Some(Budget {
            revenue: 602500,
            services_revenue: 600000,
            other_revenue: 2000,
            billable_expenses: 500,
            non_billable_expenses: 150,
            billable_hours: 150,
            non_billable_hours: 10,
        }),
        monthly_budget: Some(MonthlyBudget {
            revenue: 51220,
            services_revenue: 50000,
            other_revenue: 920,
            billable_expenses: 300,
            non_billable_expenses: 50,
            billable_hours: 10,
            non_billable_hours: 2,
        }),
    };
    assert_eq!(
        project_deserialized, project,
        "time entry did not contain the expected values"
    );

    let json_inputs = &format!(
        r#"
        {{"results": [{json_input}],"hasMore": false}}
        "#
    );
    let projects_deserialized =
        serde_json::from_str::<Projects>(json_inputs).expect("projects could not be deserialized");
    let projects = Projects {
        results: vec![project],
        has_more: false,
    };
    assert_eq!(
        projects_deserialized, projects,
        "projects did not contain the expected values"
    );
}
