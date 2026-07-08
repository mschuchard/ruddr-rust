use super::*;

#[test]
fn test_member_deserialize() {
    let json_input = r#"
        {
          "id": "3f3df320-dd95-4a42-8eae-99243fb2ea86",
          "name": "Brian Lockett",
          "email": "brian@example.com",
          "isActive": true,
          "isBillable": true,
          "loginEnabled": true,
          "invitationStatusId": "accepted",
          "employmentTypeId": "employee",
          "costMethodId": "fixed_hourly",
          "defaultRate": 100.0,
          "defaultRateCurrency": "USD",
          "activeStartDate": "2020-08-03",
          "activeEndDate": "2022-02-01",
          "timeOffAllowed": true,
          "allowedTimeOffTypes": "all",
          "timeOffApprovalMode": "member",
          "internalExpenseApprovalMode": "auto",
          "receiveMissingTimeReminders": true,
          "unsubmittedTimesheetReminders": true,
          "automaticTimesheetSubmissionConfirmation": false,
          "timesheetCapacityPolicy": "unrestricted",
          "internalId": "12345",
          "internalNotes": "Primary location: Atlanta, GA",
          "createdAt": "2020-08-03T21:00:16.370Z",
          "trackTimeByDuration": true,
          "trackTimeByTimeRange": false,
          "securityRole": {
            "id": "e31c9f7e-98cb-4ce7-81df-cc46bd9eb94f",
            "name": "Workspace Admin"
          },
          "jobTitle": {
            "id": "5a2537bd-7fc6-4d68-b639-4ec79a7fda58",
            "name": "HTML Developer"
          },
          "level": {
            "id": "a1b2c3d4-e5f6-4789-ab01-234567890abc",
            "name": "Senior"
          },
          "discipline": {
            "id": "996bf40b-d856-4c8c-b14a-f41b3a015c5b",
            "name": "Software Engineering"
          },
          "businessUnit": {
            "id": "9d6f1c2a-3b8e-4f57-ad21-7e0c4f9b8d34",
            "name": "North America"
          },
          "practice": {
            "id": "400a0c5c-32f1-41f5-a51d-e90807254965",
            "name": "Retail"
          },
          "location": {
            "id": "75e6ed16-2cdb-466e-88ad-ceeb0663ddb2",
            "name": "North America"
          },
          "manager": {
            "id": "5de6d319-ae80-4484-8b3d-0a9ff9580292",
            "name": "Steven Rodriguez"
          },
          "timeOffApprover": {
            "id": "b6816355-8945-40aa-b798-b0d6fd89e437",
            "name": "Edna Blumer"
          },
          "internalExpenseApprover": {
            "id": "82866d0d-ab62-43cd-92b8-a30ecc78bd89",
            "name": "Priya Patel"
          },
          "holidaySchedule": {
            "id": "321c155b-147e-4bd2-8aab-b9c3a3afeab1",
            "name": "U.S. Holidays"
          },
          "tags": [
            {
              "id": "4c8d3f42-6efd-4a7e-85ca-d43164db0ab2",
              "name": "Atlanta Office"
            },
            {
              "id": "5533897a-450f-42b0-a419-aa3142dd9aad",
              "name": "Buenos Aires Office"
            }
          ],
          "certifications": [
            {
              "id": "d0e1f2a3-b4c5-4678-34ab-123456789d45",
              "name": "AWS Certified Solutions Architect"
            }
          ],
          "skills": [
            {
              "id": "802c3214-59b4-4f9b-9eb1-4da2674becc3",
              "name": "JavaScript"
            },
            {
              "id": "efae4ed9-6967-4f76-9946-7b9b1008fe5e",
              "name": "HTML"
            },
            {
              "id": "828f3a35-1317-49fb-acad-419a62f74d44",
              "name": "CSS"
            }
          ],
          "timeOffTypes": [
            {
            "id": "8fc28b3d-e179-4193-bbdd-09387be8a1e9",
            "name": "Holiday"
            },
            {
            "id": "e663875e-5c11-4928-b194-66f2174740b7",
            "name": "Other Leave"
            }
          ],
          "availabilityPeriods": [
            {
              "id": "6675130f-d975-45ab-9971-4405062a9e92",
              "start": "2020-08-03",
              "end": "2022-02-01",
              "hoursPerDay": [
                8,
                8,
                8,
                8,
                8,
                0,
                0
              ]
            }
          ],
          "costPeriods": [
            {
              "id": "ae7268fd-2836-4437-b592-86289333e205",
              "start": "2020-08-03",
              "end": "2022-02-01",
              "currency": "USD",
              "costMethodId": "fixed_hourly",
              "costPerHour": 65,
              "overheadCostPerHour": 20,
              "totalCostPerHour": 85,
              "costPerMonth": null,
              "overheadCostPerMonth": null,
              "totalCostPerMonth": null,
              "currencyName": "US Dollar"
            }
          ],
          "utilizationTargetPeriods": [
            {
              "id": "f77df409-f68b-4a98-a6c6-0fa97d523e20",
              "start": "2020-08-03",
              "end": "2022-02-01",
              "targetPercentage": 80
            }
          ],
          "forbidTimesheetSubmissionWhenBelowCapacity": false
        }"#;
    let member_deserialized =
        serde_json::from_str::<Member>(json_input).expect("member could not be deserialized");
    let member = Member {
        id: types::UUID(String::from("3f3df320-dd95-4a42-8eae-99243fb2ea86")),
        name: String::from("Brian Lockett"),
        email: String::from("brian@example.com"),
        is_active: true,
        is_billable: true,
        login_enabled: true,
        invitation_status_id: InvitationStatus::Accepted,
        employment_type_id: EmploymentType::Employee,
        cost_method_id: Some(CostMethod::FixedHourly),
        default_rate: Some(100.0),
        default_rate_currency: Some(String::from("USD")),
        active_start_date: Some(types::Date(String::from("2020-08-03"))),
        active_end_date: Some(types::Date(String::from("2022-02-01"))),
        time_off_allowed: true,
        allowed_time_off_types: AllowedTimeOffTypes::All,
        time_off_approval_mode: TimeOffApprovalMode::Member,
        internal_expense_approval_mode: InternalExpenseApprovalMode::Auto,
        receive_missing_time_reminders: true,
        unsubmitted_timesheet_reminders: true,
        automatic_timesheet_submission_confirmation: false,
        timesheet_capacity_policy: TimesheetCapacityPolicy::Unrestricted,
        internal_id: Some(String::from("12345")),
        internal_notes: Some(String::from("Primary location: Atlanta, GA")),
        created_at: types::Timestamp(String::from("2020-08-03T21:00:16.370Z")),
        track_time_by_duration: true,
        track_time_by_time_range: false,
        security_role: shared::Entity {
            id: types::UUID(String::from("e31c9f7e-98cb-4ce7-81df-cc46bd9eb94f")),
            name: String::from("Workspace Admin"),
        },
        job_title: Some(shared::Entity {
            id: types::UUID(String::from("5a2537bd-7fc6-4d68-b639-4ec79a7fda58")),
            name: String::from("HTML Developer"),
        }),
        level: Some(shared::Entity {
            id: types::UUID(String::from("a1b2c3d4-e5f6-4789-ab01-234567890abc")),
            name: String::from("Senior"),
        }),
        discipline: Some(shared::Entity {
            id: types::UUID(String::from("996bf40b-d856-4c8c-b14a-f41b3a015c5b")),
            name: String::from("Software Engineering"),
        }),
        business_unit: Some(shared::Entity {
            id: types::UUID(String::from("9d6f1c2a-3b8e-4f57-ad21-7e0c4f9b8d34")),
            name: String::from("North America"),
        }),
        practice: Some(shared::Entity {
            id: types::UUID(String::from("400a0c5c-32f1-41f5-a51d-e90807254965")),
            name: String::from("Retail"),
        }),
        location: Some(shared::Entity {
            id: types::UUID(String::from("75e6ed16-2cdb-466e-88ad-ceeb0663ddb2")),
            name: String::from("North America"),
        }),
        manager: Some(shared::Entity {
            id: types::UUID(String::from("5de6d319-ae80-4484-8b3d-0a9ff9580292")),
            name: String::from("Steven Rodriguez"),
        }),
        time_off_approver: Some(shared::Entity {
            id: types::UUID(String::from("b6816355-8945-40aa-b798-b0d6fd89e437")),
            name: String::from("Edna Blumer"),
        }),
        internal_expense_approver: Some(shared::Entity {
            id: types::UUID(String::from("82866d0d-ab62-43cd-92b8-a30ecc78bd89")),
            name: String::from("Priya Patel"),
        }),
        holiday_schedule: Some(shared::Entity {
            id: types::UUID(String::from("321c155b-147e-4bd2-8aab-b9c3a3afeab1")),
            name: String::from("U.S. Holidays"),
        }),
        tags: vec![
            shared::Entity {
                id: types::UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")),
                name: String::from("Atlanta Office"),
            },
            shared::Entity {
                id: types::UUID(String::from("5533897a-450f-42b0-a419-aa3142dd9aad")),
                name: String::from("Buenos Aires Office"),
            },
        ],
        certifications: vec![shared::Entity {
            id: types::UUID(String::from("d0e1f2a3-b4c5-4678-34ab-123456789d45")),
            name: String::from("AWS Certified Solutions Architect"),
        }],
        skills: vec![
            shared::Entity {
                id: types::UUID(String::from("802c3214-59b4-4f9b-9eb1-4da2674becc3")),
                name: String::from("JavaScript"),
            },
            shared::Entity {
                id: types::UUID(String::from("efae4ed9-6967-4f76-9946-7b9b1008fe5e")),
                name: String::from("HTML"),
            },
            shared::Entity {
                id: types::UUID(String::from("828f3a35-1317-49fb-acad-419a62f74d44")),
                name: String::from("CSS"),
            },
        ],
        time_off_types: vec![
            shared::Entity {
                id: types::UUID(String::from("8fc28b3d-e179-4193-bbdd-09387be8a1e9")),
                name: String::from("Holiday"),
            },
            shared::Entity {
                id: types::UUID(String::from("e663875e-5c11-4928-b194-66f2174740b7")),
                name: String::from("Other Leave"),
            },
        ],
        availability_periods: vec![AvailabilityPeriod {
            id: types::UUID(String::from("6675130f-d975-45ab-9971-4405062a9e92")),
            start: Some(types::Date(String::from("2020-08-03"))),
            end: Some(types::Date(String::from("2022-02-01"))),
            hours_per_day: vec![8, 8, 8, 8, 8, 0, 0],
        }],
        cost_periods: vec![CostPeriod {
            id: types::UUID(String::from("ae7268fd-2836-4437-b592-86289333e205")),
            start: Some(types::Date(String::from("2020-08-03"))),
            end: Some(types::Date(String::from("2022-02-01"))),
            currency: String::from("USD"),
            cost_method_id: CostMethod::FixedHourly,
            cost_per_hour: Some(65.0),
            overhead_cost_per_hour: Some(20.0),
            total_cost_per_hour: Some(85.0),
            cost_per_month: None,
            overhead_cost_per_month: None,
            total_cost_per_month: None,
            currency_name: String::from("US Dollar"),
        }],
        utilization_target_periods: Some(vec![UtilizationTargetPeriod {
            id: types::UUID(String::from("f77df409-f68b-4a98-a6c6-0fa97d523e20")),
            start: Some(types::Date(String::from("2020-08-03"))),
            end: Some(types::Date(String::from("2022-02-01"))),
            target_percentage: Some(80.0),
        }]),
        forbid_timesheet_submission_when_below_capacity: Some(false),
    };
    assert_eq!(
        member_deserialized, member,
        "member did not contain the expected values"
    );

    let json_inputs = &format!(
        r#"
        {{"results": [{json_input}],"hasMore": false}}
        "#
    );
    let members_deserialized =
        serde_json::from_str::<Members>(json_inputs).expect("members could not be deserialized");
    let members = Members {
        results: vec![member],
        has_more: false,
    };
    assert_eq!(
        members_deserialized, members,
        "members did not contain the expected values"
    );
}
