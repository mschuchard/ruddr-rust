//! # Allocation
//!
//! `model::allocation` is a model for the Ruddr Allocation object
use crate::model::types;
use serde::Deserialize;

/// Model for Allocations used with List operations
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocations {
    pub(crate) results: Vec<Allocation>,
    pub(crate) has_more: bool,
}

/// Model for Allocation used with Read operations
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub(crate) id: types::UUID,
    pub(crate) resource_type_id: ResourceType,
    pub(crate) assignment_type_id: AssignmentType,
    pub(crate) start: types::Date,
    pub(crate) end: types::Date,
    pub(crate) unit: Unit,
    pub(crate) hours_per_day: Option<i64>,
    pub(crate) hours_per_week: Option<i64>,
    pub(crate) hours_per_month: Option<i64>,
    pub(crate) total_hours: i64,
    pub(crate) is_billable: bool,
    pub(crate) notes: String,
    pub(crate) read_only: bool,
    pub(crate) entity: Entity,
    pub(crate) created_at: types::Timestamp,
    pub(crate) member: Option<Member>,
    pub(crate) placeholder: Option<Placeholder>,
    pub(crate) project: Option<Project>,
    pub(crate) role: Option<Role>,
    pub(crate) task: Option<Task>,
    pub(crate) time_off_type: Option<TimeOffType>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Member {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Placeholder {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Project {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
    pub(crate) client: Client,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Client {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Role {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Task {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TimeOffType {
    pub(crate) id: types::UUID,
    pub(crate) name: String,
}

// custom types: enum
#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ResourceType {
    Member,
    Placeholder,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AssignmentType {
    Project,
    TimeOff,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Unit {
    Day,
    Week,
    Month,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Entity {
    Allocation,
    TimeEntry,
    Holiday,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_deserialize() {
        let json_input = r#"
        {
          "id": "212b8272-ed2a-4a91-950a-8a06b3546144",
          "resourceTypeId": "placeholder",
          "assignmentTypeId": "project",
          "start": "2022-06-01",
          "end": "2022-08-31",
          "unit": "day",
          "hoursPerDay": 8,
          "hoursPerWeek": null,
          "hoursPerMonth": null,
          "totalHours": 528,
          "isBillable": true,
          "notes": "Resourcing the PM for Joe's Shop.",
          "readOnly": false,
          "entity": "allocation",
          "createdAt": "2022-03-02T17:40:03.633Z",
          "member": {
            "id": "ec5543de-3b0f-47a0-b8ef-a6e18dc4b885",
            "name": "John Smith"
          },  
          "placeholder": {
            "id": "e804cd57-8b98-437d-86f4-90e920a41774",
            "name": "Project Manager"
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
          }
        }"#;
        let allocation_deserialized = serde_json::from_str::<Allocation>(json_input)
            .expect("allocation could not be deserialized");
        let allocation = Allocation {
            id: types::UUID(String::from("212b8272-ed2a-4a91-950a-8a06b3546144")),
            resource_type_id: ResourceType::Placeholder,
            assignment_type_id: AssignmentType::Project,
            start: types::Date(String::from("2022-06-01")),
            end: types::Date(String::from("2022-08-31")),
            unit: Unit::Day,
            hours_per_day: Some(8),
            hours_per_week: None,
            hours_per_month: None,
            total_hours: 528,
            is_billable: true,
            notes: String::from("Resourcing the PM for Joe's Shop."),
            read_only: false,
            entity: Entity::Allocation,
            created_at: types::Timestamp(String::from("2022-03-02T17:40:03.633Z")),
            member: Some(Member {
                id: types::UUID(String::from("ec5543de-3b0f-47a0-b8ef-a6e18dc4b885")),
                name: String::from("John Smith"),
            }),
            placeholder: Some(Placeholder {
                id: types::UUID(String::from("e804cd57-8b98-437d-86f4-90e920a41774")),
                name: String::from("Project Manager"),
            }),
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
        };
        assert_eq!(
            allocation_deserialized, allocation,
            "allocation did not contain the expected values"
        );

        let json_inputs = &format!(
            r#"
        {{"results": [{json_input}],"hasMore": false}}
        "#
        );
        let allocations_deserialized = serde_json::from_str::<Allocations>(json_inputs)
            .expect("allocations could not be deserialized");
        let allocations = Allocations {
            results: vec![allocation],
            has_more: false,
        };
        assert_eq!(
            allocations_deserialized, allocations,
            "allocations did not contain the expected values"
        );
    }
}
