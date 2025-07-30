use super::*;

#[test]
fn test_cost_deserialize() {
    let json_input = r#"
        {
          "costPerHour": 50,
          "id": "b3a100b0-8e71-4f39-9d96-32f11838aa8c",
          "start": "2024-01-01",
          "createdAt": "2024-11-18T18:29:33.415Z",
          "overheadCostPerHour": 15.5,
          "currency": "USD",
          "isDefault": false,
          "end": "2024-12-31",
          "totalCostPerHour": 65.5
        }"#;
    let cost_deserialized =
        serde_json::from_str::<Cost>(json_input).expect("cost could not be deserialized");
    let cost = Cost {
        cost_per_hour: 50,
        id: types::UUID(String::from("b3a100b0-8e71-4f39-9d96-32f11838aa8c")),
        start: types::Date(String::from("2024-01-01")),
        created_at: types::Timestamp(String::from("2024-11-18T18:29:33.415Z")),
        overhead_cost_per_hour: 15.5,
        currency: String::from("USD"),
        is_default: false,
        end: types::Date(String::from("2024-12-31")),
        total_cost_per_hour: 65.5,
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
