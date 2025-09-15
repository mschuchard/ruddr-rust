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
          "invoiceDetailsSource": "custom",
          "invoiceSubject": "Invoice for Professional Services",
          "invoiceNotes": "Please remit payment via ACH to Bank: 009235923",
          "invoiceEmailSubject": "Invoice Ready for Your Review - Payment via ACH",
          "invoiceEmailBody": "Hi, \n\nAn invoice has been prepared for your review. Please remit payment via ACH to Bank: 009235923. \n\nThank you for your business!",
          "isInternal": false,
          "recordStatusId": "active",
          "createdAt": "2022-02-24T16:08:18.640Z",
          "practice": {
            "id": "40f95471-7f7c-4ffa-b838-8dcccab0f54a",
            "name": "Digital Transformation"
          },
          "invoicePaymentTerm": {
            "id": "83b13634-4de2-4744-ab9e-61cf13038657",
            "name": "Net-30"
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
          ],
          "useWorkspaceInvoiceDetails": false,
          "businessUnit": {
            "id": "0e8351ea-6b3c-4307-97cc-196448de0ef1",
            "name": "EU"
          }
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
        invoice_details_source: InvoiceDetailsSource::Custom,
        invoice_subject: String::from("Invoice for Professional Services"),
        invoice_email_subject: String::from("Invoice Ready for Your Review - Payment via ACH"),
        invoice_email_body: String::from(
            "Hi, \n\nAn invoice has been prepared for your review. Please remit payment via ACH to Bank: 009235923. \n\nThank you for your business!",
        ),
        use_workspace_invoice_details: false,
        invoice_notes: String::from("Please remit payment via ACH to Bank: 009235923"),
        is_internal: false,
        record_status_id: RecordStatus::Active,
        created_at: types::Timestamp(String::from("2022-02-24T16:08:18.640Z")),
        practice: Some(shared::Entity {
            id: types::UUID(String::from("40f95471-7f7c-4ffa-b838-8dcccab0f54a")),
            name: String::from("Digital Transformation"),
        }),
        invoice_payment_term: shared::Entity {
            id: types::UUID(String::from("83b13634-4de2-4744-ab9e-61cf13038657")),
            name: String::from("Net-30"),
        },
        owner: shared::Entity {
            id: types::UUID(String::from("db010cff-a6f6-4c4e-8160-b6b7562865ff")),
            name: String::from("Cameron Howe"),
        },
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
        business_unit: Some(shared::Entity {
            id: types::UUID(String::from("0e8351ea-6b3c-4307-97cc-196448de0ef1")),
            name: String::from("EU"),
        }),
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
