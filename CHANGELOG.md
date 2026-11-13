### 1.3.0 (Next)
- Safely encode query parameters during requests.
- Fix reversed `start_date` and `end_date` parameters in `allocations` interface.
- Add `modified_at_after`, `modified_at_on_after`, `modified_at_before`, and `modified_at_on_before` filters to `time_entries` and `allocations` interfaces.
- Add `project`, `start_before`, and `end_after` filters to `allocations` interface.
- Improve date and time validation.

### 1.2.0
- Move `enums` back to respective models because `enum` now generally `pub`.
- Add optional `limit` parameter to interface functions instead of hardcoded to 100.

### 1.1.3
- Update models and interfaces for API changes.

### 1.1.2
- Add error type for custom type conversion errors.
- Leverage `reqwest::Error` type for client errors.

### 1.1.1
- Restore model deserialization validation.
- Derive `Clone` and `Serialize` for custom types.
- Derive `Serialize` for struct models.
- Publicize struct models.

### 1.1.0
- Add support for project roles.
- Update `allocation`, `project`, and `time` models for API updates.
- Add new `time` custom type to support `hh:mm`.

### 1.0.0
- First official release.

### 0.4.1
- Second release candidate.

### 0.4.0
- Release candidate.

### 0.3.1
- Fixes against second beta release.

### 0.3.0
- Second beta release.

### 0.2.0
- Initial beta release.

### 0.1.0
- Initial alpha release.