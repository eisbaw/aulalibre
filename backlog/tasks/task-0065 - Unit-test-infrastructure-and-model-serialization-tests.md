---
id: TASK-0065
title: Unit test infrastructure and model serialization tests
status: To Do
assignee: []
created_date: '2026-03-18 16:14'
labels:
  - rust
  - testing
dependencies:
  - TASK-0042
  - TASK-0043
  - TASK-0044
  - TASK-0045
  - TASK-0046
  - TASK-0047
  - TASK-0048
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up unit test infrastructure for the aula-api crate. Create test fixtures with sample JSON responses captured from API analysis. Write serde deserialization tests for all model types to verify that JSON from the Aula API deserializes correctly into Rust structs. This catches field naming mismatches, missing Optional annotations, and enum value mapping errors. Each domain module should have its own test module with sample fixtures.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Test fixtures directory with sample JSON for each API response type
- [ ] #2 Deserialization tests for profile/institution models
- [ ] #3 Deserialization tests for messaging models
- [ ] #4 Deserialization tests for calendar models
- [ ] #5 Deserialization tests for presence, post, gallery, document, notification models
- [ ] #6 Enum round-trip serialization tests for all 136 enum types
- [ ] #7 All tests pass with cargo test
<!-- AC:END -->
