use super::*;

#[test]
fn test_client_deserialize() {
    let json_input = r#"
        {
          "id": "4cacdf11-71d1-4fbb-90ee-b091803581b0",
          "key": "joes-shop",
          "name": "Joe's Shop",
          "code": "JOE",
          "currency": "USD",
          "notes": "Originally based out of Colorado.",
          "emails": [
            "joe@joesshop.com",
            "jane@joesshop.com"
          ],
          "streetAddress": "500 Main Street \nAtlanta, GA 43003",
          "useWorkspaceInvoiceDetails": false,
          "paymentTermsId": "net_15",
          "invoiceNotes": "Please remit payment via ACH to Bank: 009235923",
          "isInternal": false,
          "recordStatusId": "active",
          "createdAt": "2022-02-24T16:08:18.640Z",
          "practice": {
            "id": "40f95471-7f7c-4ffa-b838-8dcccab0f54a",
            "name": "Digital Transformation"
          },
          "owner": {
            "id": "db010cff-a6f6-4c4e-8160-b6b7562865ff",
            "name": "Cameron Howe"
          },
          "tags": [
            {
              "id": "8670e0fd-bd7a-457e-bec9-eff2b1c12b78",
              "name": "Tier 1 Client"
            },
            {
              "id": "032901d9-4a10-4ff7-af3a-a04ff6e6e606",
              "name": "Mid-Atlantic Region"
            }
          ]
        }"#;
    let client_deserialized =
        serde_json::from_str::<Client>(json_input).expect("client could not be deserialized");
    let client = Client {
        id: types::UUID(String::from("4cacdf11-71d1-4fbb-90ee-b091803581b0")),
        key: String::from("joes-shop"),
        name: String::from("Joe's Shop"),
        code: String::from("JOE"),
        currency: String::from("USD"),
        notes: String::from("Originally based out of Colorado."),
        emails: vec![
            String::from("joe@joesshop.com"),
            String::from("jane@joesshop.com"),
        ],
        street_address: String::from("500 Main Street \nAtlanta, GA 43003"),
        use_workspace_invoice_details: false,
        payment_terms_id: PaymentTerms::Net15,
        invoice_notes: String::from("Please remit payment via ACH to Bank: 009235923"),
        is_internal: false,
        record_status_id: RecordStatus::Active,
        created_at: types::Timestamp(String::from("2022-02-24T16:08:18.640Z")),
        practice: Some(Practice {
            id: types::UUID(String::from("40f95471-7f7c-4ffa-b838-8dcccab0f54a")),
            name: String::from("Digital Transformation"),
        }),
        owner: Owner {
            id: types::UUID(String::from("db010cff-a6f6-4c4e-8160-b6b7562865ff")),
            name: String::from("Cameron Howe"),
        },
        tags: vec![
            Tag {
                id: types::UUID(String::from("8670e0fd-bd7a-457e-bec9-eff2b1c12b78")),
                name: String::from("Tier 1 Client"),
            },
            Tag {
                id: types::UUID(String::from("032901d9-4a10-4ff7-af3a-a04ff6e6e606")),
                name: String::from("Mid-Atlantic Region"),
            },
        ],
    };
    assert_eq!(
        client_deserialized, client,
        "client did not contain the expected values"
    );

    let json_inputs = &format!(
        r#"
        {{"results": [{json_input}],"hasMore": false}}
        "#
    );
    let clients_deserialized =
        serde_json::from_str::<Clients>(json_inputs).expect("clients could not be deserialized");
    let clients = Clients {
        results: vec![client],
        has_more: false,
    };
    assert_eq!(
        clients_deserialized, clients,
        "clients did not contain the expected values"
    );
}
