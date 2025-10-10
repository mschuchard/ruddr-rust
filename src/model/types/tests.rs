use super::*;

#[test]
fn test_date_new() {
    assert_eq!(
        Date(String::from("1234-56-78")),
        Date::new(String::from("1234-56-78")).expect("date conversion failed")
    )
}

#[test]
fn test_date_new_error() {
    assert_eq!(
        Date::new(String::from("99-99-9999")).unwrap_err(),
        "invalid date: 99-99-9999"
    );
}

#[test]
fn test_date_from_str() {
    assert_eq!(
        Date(String::from("1234-56-78")),
        Date::try_from("1234-56-78").expect("date conversion failed")
    )
}

#[test]
fn test_date_from_string() {
    assert_eq!(
        Date::try_from(String::from("1234-56-78")).expect("date conversion failed"),
        Date(String::from("1234-56-78")),
    )
}

#[test]
fn test_date_to_string() {
    assert_eq!(
        String::from("1234-56-78"),
        String::from(Date(String::from("1234-56-78")))
    )
}

#[test]
fn test_date_to_str() {
    assert_eq!(
        "1234-56-78",
        &String::from(Date(String::from("1234-56-78")))
    )
}

#[test]
fn test_date_deserialize() {
    assert_eq!(
        Date(String::from("1234-56-78")),
        serde_json::from_str::<Date>("\"1234-56-78\"").expect("date could not be deserialized")
    )
}

#[test]
fn test_date_display() {
    assert_eq!(
        String::from("1234-56-78"),
        format!("{}", Date(String::from("1234-56-78")))
    )
}

#[test]
fn test_timestamp_new() {
    assert_eq!(
        Timestamp(String::from("1234-56-78T12:34:56.789Z")),
        Timestamp::new(String::from("1234-56-78T12:34:56.789Z"))
            .expect("timestamp conversion failed")
    )
}

#[test]
fn test_timestamp_new_error() {
    assert_eq!(
        Timestamp::new(String::from("99-99-9999")).unwrap_err(),
        "invalid timestamp: 99-99-9999"
    );
}

#[test]
fn test_timestamp_from_str() {
    assert_eq!(
        Timestamp(String::from("1234-56-78T12:34:56.789Z")),
        Timestamp::try_from("1234-56-78T12:34:56.789Z").expect("timestamp conversion failed")
    )
}

#[test]
fn test_timestamp_from_string() {
    assert_eq!(
        Timestamp::try_from(String::from("1234-56-78T12:34:56.789Z"))
            .expect("timestamp conversion failed"),
        Timestamp(String::from("1234-56-78T12:34:56.789Z")),
    )
}

#[test]
fn test_timestamp_to_string() {
    assert_eq!(
        String::from("1234-56-78T12:34:56.789Z"),
        String::from(Timestamp(String::from("1234-56-78T12:34:56.789Z")))
    )
}

#[test]
fn test_timestamp_to_str() {
    assert_eq!(
        "1234-56-78T12:34:56.789Z",
        &String::from(Timestamp(String::from("1234-56-78T12:34:56.789Z")))
    )
}

#[test]
fn test_timestamp_deserialize() {
    assert_eq!(
        Timestamp(String::from("1234-56-78T12:34:56.789Z")),
        serde_json::from_str::<Timestamp>("\"1234-56-78T12:34:56.789Z\"")
            .expect("timestamp could not be deserialized")
    )
}

#[test]
fn test_timestamp_display() {
    assert_eq!(
        String::from("1234-56-78T12:34:56.789Z"),
        format!("{}", Timestamp(String::from("1234-56-78T12:34:56.789Z")))
    )
}

#[test]
fn test_uuid_new() {
    assert_eq!(
        UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")),
        UUID::new(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2"))
            .expect("uuid conversion failed")
    )
}

#[test]
fn test_uuid_new_error() {
    assert_eq!(
        UUID::new(String::from("foo-bar-baz")).unwrap_err(),
        "invalid uuid: foo-bar-baz"
    );
}

#[test]
fn test_uuid_from_str() {
    assert_eq!(
        UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")),
        UUID::try_from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2").expect("uuid conversion failed"),
    )
}

#[test]
fn test_uuid_from_string() {
    assert_eq!(
        UUID::try_from(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2"))
            .expect("uuid conversion failed"),
        UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")),
    )
}

#[test]
fn test_uuid_to_string() {
    assert_eq!(
        String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2"),
        String::from(UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")))
    )
}

#[test]
fn test_uuid_to_str() {
    assert_eq!(
        "4c8d3f42-6efd-4a7e-85ca-d43164db0ab2",
        &String::from(UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")))
    )
}

#[test]
fn test_uuid_deserialize() {
    assert_eq!(
        UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2")),
        serde_json::from_str::<UUID>("\"4c8d3f42-6efd-4a7e-85ca-d43164db0ab2\"")
            .expect("uuid could not be deserialized")
    )
}

#[test]
fn test_uuid_display() {
    assert_eq!(
        String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2"),
        format!(
            "{}",
            UUID(String::from("4c8d3f42-6efd-4a7e-85ca-d43164db0ab2"))
        )
    )
}

#[test]
fn test_slug_new() {
    assert_eq!(
        Slug(String::from("vendor1-portal2")),
        Slug::new(String::from("vendor1-portal2")).expect("slug conversion failed")
    )
}

#[test]
fn test_slug_new_error() {
    assert_eq!(
        Slug::new(String::from("Foo-Bar-Baz!")).unwrap_err(),
        "invalid slug: Foo-Bar-Baz!"
    );
}

#[test]
fn test_slug_from_str() {
    assert_eq!(
        Slug(String::from("vendor1-portal2")),
        Slug::try_from("vendor1-portal2").expect("slug conversion failed"),
    )
}

#[test]
fn test_slug_from_string() {
    assert_eq!(
        Slug::try_from(String::from("vendor1-portal2")).expect("slug conversion failed"),
        Slug(String::from("vendor1-portal2")),
    )
}

#[test]
fn test_slug_to_string() {
    assert_eq!(
        String::from("vendor1-portal2"),
        String::from(Slug(String::from("vendor1-portal2")))
    )
}

#[test]
fn test_slug_to_str() {
    assert_eq!(
        "vendor1-portal2",
        &String::from(Slug(String::from("vendor1-portal2")))
    )
}

#[test]
fn test_slug_deserialize() {
    assert_eq!(
        Slug(String::from("vendor1-portal2")),
        serde_json::from_str::<Slug>("\"vendor1-portal2\"")
            .expect("slug could not be deserialized")
    )
}

#[test]
fn test_slug_display() {
    assert_eq!(
        String::from("vendor1-portal2"),
        format!("{}", Slug(String::from("vendor1-portal2")))
    )
}
