//! # Member
//!
//! `model::member` is a model for the Ruddr Member object.
use crate::model::types;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
pub struct Members {
    pub results: Vec<Member>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub id: types::UUID,
    pub name: String,
    pub email: String,
    pub is_active: bool,
    pub is_billable: bool,
    pub login_enabled: bool,
    pub invitation_status_id: InvitationStatus,
    pub employment_type_id: EmploymentType,
    pub cost_method_id: CostMethod,
    pub default_rate: f64,
    pub default_rate_currency: String,
    pub active_start_date: types::Date,
    pub active_end_date: types::Date,
    pub time_off_allowed: bool,
    pub time_off_approval_mode: TimeOffApprovalMode,
    pub receive_missing_time_reminders: bool,
    pub unsubmitted_timesheet_reminders: bool,
    pub forbid_timesheet_submission_when_below_capacity: bool,
    pub internal_id: String,
    pub internal_notes: String,
    pub created_at: types::Timestamp,
    pub security_role: SecurityRole,
    pub job_title: JobTitle,
    pub discipline: Discipline,
    pub practice: Practice,
    pub location: Location,
    pub manager: Manager,
    pub time_off_approver: TimeOffApprover,
    pub holiday_schedule: HolidaySchedule,
    pub tags: Vec<Tag>,
    pub skills: Vec<Skill>,
    pub availability_periods: Vec<AvailabilityPeriod>,
    pub cost_periods: Vec<CostPeriod>,
    pub utilization_target_periods: Vec<UtilizationTargetPeriod>,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityRole {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JobTitle {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Discipline {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Practice {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Manager {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeOffApprover {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HolidaySchedule {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub id: types::UUID,
    pub name: String,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityPeriod {
    pub id: types::UUID,
    pub start: types::Date,
    pub end: types::Date,
    pub hours_per_day: Vec<i64>,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CostPeriod {
    pub id: types::UUID,
    pub start: types::Date,
    pub end: types::Date,
    pub currency: String,
    pub cost_per_hour: i64,
    pub overhead_cost_per_hour: i64,
    pub total_cost_per_hour: i64,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UtilizationTargetPeriod {
    pub id: types::UUID,
    pub start: types::Date,
    pub end: types::Date,
    pub target_percentage: i64,
}

// custom types: enum
#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InvitationStatus {
    NotInvited,
    Invited,
    Accepted,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EmploymentType {
    Employee,
    Contractor,
    Other,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CostMethod {
    Hourly,
    Fixed,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TimeOffApprovalMode {
    Auto,
    Manager,
    Member,
}

#[cfg(test)]
mod tests {
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
  "costMethodId": "fixed",
  "defaultRate": 100.0,
  "defaultRateCurrency": "USD",
  "activeStartDate": "2020-08-03",
  "activeEndDate": "2022-02-01",
  "timeOffAllowed": true,
  "timeOffApprovalMode": "member",
  "receiveMissingTimeReminders": true,
  "unsubmittedTimesheetReminders": true,
  "forbidTimesheetSubmissionWhenBelowCapacity": false,
  "internalId": "12345",
  "internalNotes": "Primary location: Atlanta, GA",
  "createdAt": "2020-08-03T21:00:16.370Z",
  "securityRole": {
    "id": "e31c9f7e-98cb-4ce7-81df-cc46bd9eb94f",
    "name": "Workspace Admin"
  },
  "jobTitle": {
    "id": "5a2537bd-7fc6-4d68-b639-4ec79a7fda58",
    "name": "HTML Developer"
  },
  "discipline": {
    "id": "996bf40b-d856-4c8c-b14a-f41b3a015c5b",
    "name": "Software Engineering"
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
      "costPerHour": 65,
      "overheadCostPerHour": 20,
      "totalCostPerHour": 85
    }
  ],
  "utilizationTargetPeriods": [
    {
      "id": "f77df409-f68b-4a98-a6c6-0fa97d523e20",
      "start": "2020-08-03",
      "end": "2022-02-01",
      "targetPercentage": 80
    }
  ]
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
            cost_method_id: CostMethod::Fixed,
            default_rate: 100.0,
            default_rate_currency: String::from("USD"),
            active_start_date: types::Date(String::from("2020-08-03")),
            active_end_date: types::Date(String::from("2022-02-01")),
            time_off_allowed: true,
            time_off_approval_mode: TimeOffApprovalMode::Member,
            receive_missing_time_reminders: true,
            unsubmitted_timesheet_reminders: true,
            forbid_timesheet_submission_when_below_capacity: false,
            internal_id: String::from("12345"),
            internal_notes: String::from("Primary location: Atlanta, GA"),
            created_at: types::Timestamp(String::from("2020-08-03T21:00:16.370Z")),
            security_role: SecurityRole {
                id: types::UUID(String::from("e31c9f7e-98cb-4ce7-81df-cc46bd9eb94f")),
                name: String::from("Workspace Admin"),
            },
            job_title: JobTitle {
                id: types::UUID(String::from("5a2537bd-7fc6-4d68-b639-4ec79a7fda58")),
                name: String::from("HTML Developer"),
            },
            discipline: Discipline {
                id: types::UUID(String::from("996bf40b-d856-4c8c-b14a-f41b3a015c5b")),
                name: String::from("Software Engineering"),
            },
            practice: Practice {
                id: types::UUID(String::from("400a0c5c-32f1-41f5-a51d-e90807254965")),
                name: String::from("Retail"),
            },
            location: Location {
                id: types::UUID(String::from("75e6ed16-2cdb-466e-88ad-ceeb0663ddb2")),
                name: String::from("North America"),
            },
            manager: Manager {
                id: types::UUID(String::from("5de6d319-ae80-4484-8b3d-0a9ff9580292")),
                name: String::from("Steven Rodriguez"),
            },
            time_off_approver: TimeOffApprover {
                id: types::UUID(String::from("b6816355-8945-40aa-b798-b0d6fd89e437")),
                name: String::from("Edna Blumer"),
            },
            holiday_schedule: HolidaySchedule {
                id: types::UUID(String::from("321c155b-147e-4bd2-8aab-b9c3a3afeab1")),
                name: String::from("U.S. Holidays"),
            },
            tags: vec![
                Tag {
                    id: types::UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")),
                    name: String::from("Atlanta Office"),
                },
                Tag {
                    id: types::UUID(String::from("5533897a-450f-42b0-a419-aa3142dd9aad")),
                    name: String::from("Buenos Aires Office"),
                },
            ],
            skills: vec![
                Skill {
                    id: types::UUID(String::from("802c3214-59b4-4f9b-9eb1-4da2674becc3")),
                    name: String::from("JavaScript"),
                },
                Skill {
                    id: types::UUID(String::from("efae4ed9-6967-4f76-9946-7b9b1008fe5e")),
                    name: String::from("HTML"),
                },
                Skill {
                    id: types::UUID(String::from("828f3a35-1317-49fb-acad-419a62f74d44")),
                    name: String::from("CSS"),
                },
            ],
            availability_periods: vec![AvailabilityPeriod {
                id: types::UUID(String::from("6675130f-d975-45ab-9971-4405062a9e92")),
                start: types::Date(String::from("2020-08-03")),
                end: types::Date(String::from("2022-02-01")),
                hours_per_day: vec![8, 8, 8, 8, 8, 0, 0],
            }],
            cost_periods: vec![CostPeriod {
                id: types::UUID(String::from("ae7268fd-2836-4437-b592-86289333e205")),
                start: types::Date(String::from("2020-08-03")),
                end: types::Date(String::from("2022-02-01")),
                currency: String::from("USD"),
                cost_per_hour: 65,
                overhead_cost_per_hour: 20,
                total_cost_per_hour: 85,
            }],
            utilization_target_periods: vec![UtilizationTargetPeriod {
                id: types::UUID(String::from("f77df409-f68b-4a98-a6c6-0fa97d523e20")),
                start: types::Date(String::from("2020-08-03")),
                end: types::Date(String::from("2022-02-01")),
                target_percentage: 80,
            }],
        };
        assert_eq!(
            member_deserialized, member,
            "member did not contain the expected values"
        )
    }
}
