---
id: TASK-0122
title: 'Fix messages JSON deserialization: integer IDs where String expected'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-23 09:02'
updated_date: '2026-03-23 09:11'
labels:
  - aula-api
  - bug
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Aula API inconsistently returns ID fields as integers vs strings. C# client uses Newtonsoft.Json which silently coerces, but Rust serde is strict. A deserialize_optional_string_from_any helper exists in messaging.rs but is only applied to one field. Multiple Option<String> ID fields need the annotation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Apply deserialize_optional_string_from_any to all Option<String> ID fields in messaging models
- [x] #2 messages list --all succeeds without JSON errors
- [x] #3 Audit other model files for the same pattern
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create shared serde_helpers module with deserialize_optional_string_from_any
2. Remove local helper from messaging.rs, import shared one
3. Annotate all Option<String> ID fields in messaging.rs
4. Audit and annotate ID fields in notifications.rs, calendar.rs, profiles.rs, search.rs, onboarding.rs
5. Fix ThreadType snake_case alias (discovered during testing)
6. Run tests, clippy, verify messages list --all works
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed JSON deserialization failures caused by the Aula API inconsistently returning ID fields as integers vs strings.

Changes:
- Created shared `serde_helpers` module (`aula/aula-api/src/serde_helpers.rs`) with `deserialize_optional_string_from_any` helper, replacing the local copy in messaging.rs
- Applied `#[serde(default, deserialize_with = "...")]` to all `Option<String>` ID fields across 6 model files:
  - messaging.rs: thread_id, entity_id, id, last_read_message_id (4 structs)
  - notifications.rs: notification_id, element_id, id
  - calendar.rs: lesson_id (3 structs)
  - profiles.rs: user_id (2 structs), widget_id
  - search.rs: doc_id (11 structs), id (2 structs), message_id
  - onboarding.rs: user_id
- Fixed ThreadType enum to accept both camelCase and snake_case variants via serde aliases (API inconsistency discovered during testing)

Verification:
- `cargo test` passes (40 tests)
- `cargo clippy -- -D warnings` clean
- `aula-cli messages list --all` now succeeds without JSON errors
<!-- SECTION:FINAL_SUMMARY:END -->
