use super::*;

#[test]
fn test_project_deserialize() {
    let json_input = r#"
        {
          "id": "4497fa99-27a4-4509-9748-83e4399296e3",
          "name": "Vendor Portal",
          "client": {
            "id": "4cacdf11-71d1-4fbb-90ee-b091803581b0",
            "name": "Joe's Shop"
          }
        }"#;
    let project_deserialized =
        serde_json::from_str::<Project>(json_input).expect("project could not be deserialized");
    let project = Project {
        id: types::UUID(String::from("4497fa99-27a4-4509-9748-83e4399296e3")),
        name: String::from("Vendor Portal"),
        client: Entity {
            id: types::UUID(String::from("4cacdf11-71d1-4fbb-90ee-b091803581b0")),
            name: String::from("Joe's Shop"),
        },
    };
    assert_eq!(project, project_deserialized);
}

#[test]
fn test_entity_deserialize() {
    let json_input = r#"
        {
          "id": "ec5543de-3b0f-47a0-b8ef-a6e18dc4b885",
          "name": "John Smith"
        }"#;
    let entity_deserialized =
        serde_json::from_str::<Entity>(json_input).expect("entity could not be deserialized");
    let entity = Entity {
        id: types::UUID(String::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
        name: String::from("John Smith"),
    };
    assert_eq!(entity, entity_deserialized);
}
