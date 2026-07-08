use super::*;

#[test]
fn test_client_deserialize() {
    let json_input = r#"
        {
          "id": "4cacdf11-71d1-4fbb-90ee-b091803581b0",
          "key": "joes-shop",
          "name": "Joe's Shop",
          "code": null,
          "currency": "USD",
          "notes": null,
          "emails": [
            "joe@joesshop.com",
            "jane@joesshop.com"
          ],
          "streetAddress": null,
          "invoiceDetailsSource": "custom",
          "invoiceSubject": null,
          "invoiceNotes": null,
          "invoiceEmailSubject": null,
          "invoiceEmailBody": null,
          "isInternal": false,
          "recordStatusId": "active",
          "createdAt": "2022-02-24T16:08:18.640Z",
          "practice": {
            "id": "40f95471-7f7c-4ffa-b838-8dcccab0f54a",
            "name": "Digital Transformation"
          },
          "industry": {
            "id": "bc52e68a-96d9-46bd-881c-bda8d5e07053",
            "name": "Technology"
          },
          "location": {
            "id": "ae469b03-51f3-47c2-be12-a319e0ce7cd8",
            "name": "New York"
          },
          "invoicePaymentTerm": null,
          "owner": null,
          "tags": [
            {
              "id": "8670e0fd-bd7a-457e-bec9-eff2b1c12b78",
              "name": "Tier 1 Client"
            },
            {
              "id": "032901d9-4a10-4ff7-af3a-a04ff6e6e606",
              "name": "Mid-Atlantic Region"
            }
          ],
          "salesRepresentative": {
            "id": "c6f2b081-d47a-4e93-a8d5-3e1c7f9a204b",
            "name": "John Davis"
          },
          "useWorkspaceInvoiceDetails": false,
          "businessUnit": {
            "id": "0e8351ea-6b3c-4307-97cc-196448de0ef1",
            "name": "EU"
          },
          "integrations": [
            {
              "type": "xero",
              "connectionId": "9d2f6b1a-4c83-4e57-bf09-1a6d3e8c20b4",
              "externalId": "5e1a9c47-2b86-4d30-9f12-7a4c8e3b50d9"
            }
          ]
        }"#;
    let client_deserialized =
        serde_json::from_str::<Client>(json_input).expect("client could not be deserialized");
    let client = Client {
        id: types::UUID(String::from("4cacdf11-71d1-4fbb-90ee-b091803581b0")),
        key: String::from("joes-shop"),
        name: String::from("Joe's Shop"),
        code: None,
        currency: String::from("USD"),
        notes: None,
        emails: vec![
            String::from("joe@joesshop.com"),
            String::from("jane@joesshop.com"),
        ],
        street_address: None,
        invoice_details_source: InvoiceDetailsSource::Custom,
        invoice_subject: None,
        invoice_email_subject: None,
        invoice_email_body: None,
        use_workspace_invoice_details: false,
        invoice_notes: None,
        is_internal: false,
        record_status_id: RecordStatus::Active,
        created_at: types::Timestamp(String::from("2022-02-24T16:08:18.640Z")),
        practice: Some(shared::Entity {
            id: types::UUID(String::from("40f95471-7f7c-4ffa-b838-8dcccab0f54a")),
            name: String::from("Digital Transformation"),
        }),
        industry: Some(shared::Entity {
            id: types::UUID(String::from("bc52e68a-96d9-46bd-881c-bda8d5e07053")),
            name: String::from("Technology"),
        }),
        location: Some(shared::Entity {
            id: types::UUID(String::from("ae469b03-51f3-47c2-be12-a319e0ce7cd8")),
            name: String::from("New York"),
        }),
        invoice_payment_term: None,
        owner: None,
        tags: vec![
            shared::Entity {
                id: types::UUID(String::from("8670e0fd-bd7a-457e-bec9-eff2b1c12b78")),
                name: String::from("Tier 1 Client"),
            },
            shared::Entity {
                id: types::UUID(String::from("032901d9-4a10-4ff7-af3a-a04ff6e6e606")),
                name: String::from("Mid-Atlantic Region"),
            },
        ],
        sales_representative: Some(shared::Entity {
            id: types::UUID(String::from("c6f2b081-d47a-4e93-a8d5-3e1c7f9a204b")),
            name: String::from("John Davis"),
        }),
        business_unit: Some(shared::Entity {
            id: types::UUID(String::from("0e8351ea-6b3c-4307-97cc-196448de0ef1")),
            name: String::from("EU"),
        }),
        integrations: vec![Integration {
            integration_type: IntegrationType::Xero,
            connection_id: types::UUID(String::from("9d2f6b1a-4c83-4e57-bf09-1a6d3e8c20b4")),
            external_id: String::from("5e1a9c47-2b86-4d30-9f12-7a4c8e3b50d9"),
        }],
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
