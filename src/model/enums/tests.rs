use super::*;

#[test]
fn test_assignment_type_display() {
    assert_eq!(AssignmentType::TimeOff.to_string(), "time_off")
}

#[test]
fn test_status_display() {
    assert_eq!(Status::InProgress.to_string(), "in_progress")
}
