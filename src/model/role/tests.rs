use super::*;

#[test]
fn test_role_deserialize() {
    let json_input = r#"
        {
          "id": "7ad5a34a-07b7-48e9-a760-bd220d52e354",
          "name": "Project Manager",
          "isActive": true,
          "isBillable": true,
          "rate": 125,
          "createdAt": "2022-03-15T15:00:08.626Z",
          "project": {
            "id": "095e0780-48bf-472c-8deb-2fc3ebc7d90c",
            "name": "Vendor Portal",
            "client": {
              "id": "4cacdf11-71d1-4fbb-90ee-b091803581b0",
              "name": "Joe's Shop"
            }
          },
          "discipline": {
            "id": "14aa2bab-ea87-4ca3-9d5b-08aade12376e",
            "name": "Project Management"
          },
          "budget": {
            "billableHours": 150,
            "nonBillableHours": 10
          },
          "monthlyBudget": {
            "billableHours": 30,
            "nonBillableHours": 2
          }
        }"#;
    let role_deserialized =
        serde_json::from_str::<Role>(json_input).expect("role could not be deserialized");
    let role = Role {
        id: types::UUID(String::from("7ad5a34a-07b7-48e9-a760-bd220d52e354")),
        name: String::from("Project Manager"),
        is_active: true,
        is_billable: true,
        rate: Some(125.0),
        created_at: types::Timestamp(String::from("2022-03-15T15:00:08.626Z")),
        project: shared::Project {
            id: types::UUID(String::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")),
            name: String::from("Vendor Portal"),
            client: shared::Entity {
                id: types::UUID(String::from("4cacdf11-71d1-4fbb-90ee-b091803581b0")),
                name: String::from("Joe's Shop"),
            },
        },
        discipline: shared::Entity {
            id: types::UUID(String::from("14aa2bab-ea87-4ca3-9d5b-08aade12376e")),
            name: String::from("Project Management"),
        },
        budget: Some(Budget {
            billable_hours: Some(150),
            non_billable_hours: 10,
        }),
        monthly_budget: Some(Budget {
            billable_hours: Some(30),
            non_billable_hours: 2,
        }),
    };
    assert_eq!(
        role, role_deserialized,
        "role did not contain the expected values"
    );

    let json_inputs = &format!(
        r#"
        {{"results": [{json_input}],"hasMore": false}}
        "#
    );
    let roles_deserialized =
        serde_json::from_str::<Roles>(json_inputs).expect("roles could not be deserialized");
    let roles = Roles {
        results: vec![role],
        has_more: false,
    };
    assert_eq!(
        roles_deserialized, roles,
        "roles did not contain the expected values"
    );
}
