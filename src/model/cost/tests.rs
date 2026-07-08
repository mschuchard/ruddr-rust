use super::*;

#[test]
fn test_cost_deserialize() {
    let json_input = r#"
        {
          "id": "b3a100b0-8e71-4f39-9d96-32f11838aa8c",
          "isDefault": false,
          "start": "2025-01-01",
          "end": null,
          "currency": "USD",
          "costMethodId": "fixed_monthly",
          "costPerHour": null,
          "overheadCostPerHour": null,
          "totalCostPerHour": null,
          "costPerMonth": 12000,
          "overheadCostPerMonth": 3000,
          "totalCostPerMonth": 15000,
          "createdAt": "2025-01-01T09:00:00.000Z"
        }"#;
    let cost_deserialized =
        serde_json::from_str::<Cost>(json_input).expect("cost could not be deserialized");
    let cost = Cost {
        id: types::UUID(String::from("b3a100b0-8e71-4f39-9d96-32f11838aa8c")),
        is_default: false,
        start: types::Date(String::from("2025-01-01")),
        end: None,
        currency: String::from("USD"),
        cost_method_id: CostMethod::FixedMonthly,
        cost_per_hour: None,
        overhead_cost_per_hour: None,
        total_cost_per_hour: None,
        cost_per_month: Some(12000.0),
        overhead_cost_per_month: Some(3000.0),
        total_cost_per_month: Some(15000.0),
        created_at: types::Timestamp(String::from("2025-01-01T09:00:00.000Z")),
    };
    assert_eq!(
        cost_deserialized, cost,
        "cost did not contain the expected values"
    );

    let json_inputs = &format!(
        r#"
        {{"results": [{json_input}],"hasMore": false}}
        "#
    );
    let costs_deserialized =
        serde_json::from_str::<Costs>(json_inputs).expect("costs could not be deserialized");
    let costs = Costs {
        results: vec![cost],
        has_more: false,
    };
    assert_eq!(
        costs_deserialized, costs,
        "costs did not contain the expected values"
    );
}
