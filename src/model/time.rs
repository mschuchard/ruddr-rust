//! # Time
//!
//! `model::time` is a model for the Ruddr Time Entry object. This module is not publically accessible, but the structs and members are public for reading from `interface::time` returns.
//! https://ruddr.readme.io/reference/time-entry-object
use crate::model::types;
use serde::{Deserialize, Serialize};

/// Model for TimeEntries used with List operations
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntries {
    pub results: Vec<TimeEntry>,
    pub has_more: bool,
}

/// Model for TimeEntry used with Read operations
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    pub id: types::UUID,
    pub type_id: Type,
    pub status_id: Status,
    pub date: types::Date,
    pub minutes: i64,
    pub timer_started_at: Option<types::Timestamp>,
    pub notes: String,
    pub is_billable: bool,
    pub invoiced: bool,
    pub created_at: types::Timestamp,
    pub member: Member,
    pub project: Option<Project>,
    pub role: Option<Role>,
    pub task: Option<Task>,
    pub time_off_type: Option<TimeOffType>,
    pub invoice: Option<Invoice>,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub id: types::UUID,
    pub name: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: types::UUID,
    pub name: String,
    pub client: Client,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub id: types::UUID,
    pub name: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: types::UUID,
    pub name: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: types::UUID,
    pub name: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeOffType {
    pub id: types::UUID,
    pub name: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: types::UUID,
    pub number: String,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    ProjectTime,
    TimeOff,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    NotSubmitted,
    PendingApproval,
    Approved,
    Rejected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_deserialize() {
        let json_input = r#"
        {
          "id": "4497fa99-27a4-4509-9748-83e4399296e3",
          "typeId": "project_time",
          "statusId": "approved",
          "date": "2022-03-08",
          "minutes": 120,
          "timerStartedAt": "2022-03-10T17:50:48.808Z",
          "notes": "Write up the weekly status report.",
          "isBillable": true,
          "invoiced": true,
          "createdAt": "2022-03-11T16:13:40.715Z",
          "member": {
            "id": "ec5543de-3b0f-47a0-b8ef-a6e18dc4b885",
            "name": "John Smith"
          },
          "project": {
            "id": "095e0780-48bf-472c-8deb-2fc3ebc7d90c",
            "name": "Vendor Portal",
            "client": {
              "id": "4cacdf11-71d1-4fbb-90ee-b091803581b0",
              "name": "Joe's Shop"
            }
          },
          "role": {
            "id": "7ad5a34a-07b7-48e9-a760-bd220d52e354",
            "name": "Project Manager"
          },
          "task": {
            "id": "9a7097a0-d71e-4ed2-9bc3-2dd7d797edc4",
            "name": "Project Status Reporting"
          },
          "timeOffType": {
            "id": "8fc28b3d-e179-4193-bbdd-09387be8a1e9",
            "name": "Holiday"
          },
          "invoice": {
            "id": "76a4f846-3e3b-43a2-bd9b-f5afabdad447",
            "number": "INV-0001"
          }
        }"#;
        let time_entry_deserialized = serde_json::from_str::<TimeEntry>(json_input)
            .expect("time entry could not be deserialized");
        let time_entry = TimeEntry {
            id: types::UUID(String::from("4497fa99-27a4-4509-9748-83e4399296e3")),
            type_id: Type::ProjectTime,
            status_id: Status::Approved,
            date: types::Date(String::from("2022-03-08")),
            minutes: 120,
            timer_started_at: Some(types::Timestamp(String::from("2022-03-10T17:50:48.808Z"))),
            notes: String::from("Write up the weekly status report."),
            is_billable: true,
            invoiced: true,
            created_at: types::Timestamp(String::from("2022-03-11T16:13:40.715Z")),
            member: Member {
                id: types::UUID(String::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
                name: String::from("John Smith"),
            },
            project: Some(Project {
                id: types::UUID(String::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c")),
                name: String::from("Vendor Portal"),
                client: Client {
                    id: types::UUID(String::from("4cacdf11-71d1-4fbb-90ee-b091803581b0")),
                    name: String::from("Joe's Shop"),
                },
            }),
            role: Some(Role {
                id: types::UUID(String::from("7ad5a34a-07b7-48e9-a760-bd220d52e354")),
                name: String::from("Project Manager"),
            }),
            task: Some(Task {
                id: types::UUID(String::from("9a7097a0-d71e-4ed2-9bc3-2dd7d797edc4")),
                name: String::from("Project Status Reporting"),
            }),
            time_off_type: Some(TimeOffType {
                id: types::UUID(String::from("8fc28b3d-e179-4193-bbdd-09387be8a1e9")),
                name: String::from("Holiday"),
            }),
            invoice: Some(Invoice {
                id: types::UUID(String::from("76a4f846-3e3b-43a2-bd9b-f5afabdad447")),
                number: String::from("INV-0001"),
            }),
        };
        assert_eq!(
            time_entry_deserialized, time_entry,
            "time entry did not contain the expected values"
        );

        let json_inputs = &format!(
            r#"
        {{"results": [{json_input}],"hasMore": false}}
        "#
        );
        let time_entries_deserialized = serde_json::from_str::<TimeEntries>(json_inputs)
            .expect("time_entries could not be deserialized");
        let time_entries = TimeEntries {
            results: vec![time_entry],
            has_more: false,
        };
        assert_eq!(
            time_entries_deserialized, time_entries,
            "time_entries did not contain the expected values"
        );
    }
}
