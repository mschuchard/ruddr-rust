use super::*;

#[test]
fn test_utilization_deserialize() {
    let json_input = r#"
        {
          "id": "8e6d6316-5bc2-4135-b99c-f604f29051ab",
          "start": "2024-11-01",
          "targetPercentage": 85,
          "createdAt": "2024-11-08T00:50:42.006Z",
          "isDefault": false,
          "end": "2025-10-31"
        }"#;
    let utilization_deserialized = serde_json::from_str::<Utilization>(json_input)
        .expect("utilization could not be deserialized");
    let utilization = Utilization {
        id: types::UUID(String::from("8e6d6316-5bc2-4135-b99c-f604f29051ab")),
        start: types::Date(String::from("2024-11-01")),
        target_percentage: 85.0,
        created_at: types::Timestamp(String::from("2024-11-08T00:50:42.006Z")),
        is_default: false,
        end: types::Date(String::from("2025-10-31")),
    };
    assert_eq!(
        utilization_deserialized, utilization,
        "utilization did not contain the expected values"
    );

    let json_inputs = &format!(
        r#"
        {{"results": [{json_input}],"hasMore": false}}
        "#
    );
    let utilizations_deserialized = serde_json::from_str::<Utilizations>(json_inputs)
        .expect("utilizations could not be deserialized");
    let utilizations = Utilizations {
        results: vec![utilization],
        has_more: false,
    };
    assert_eq!(
        utilizations_deserialized, utilizations,
        "utilizations did not contain the expected values"
    );
}
