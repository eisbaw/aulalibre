---
id: TASK-0121
title: 'Fix messages JSON deserialization: integer IDs where String expected'
status: To Do
assignee: []
created_date: '2026-03-23 09:02'
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
- [ ] #1 Apply deserialize_optional_string_from_any to all Option<String> ID fields in messaging models
- [ ] #2 messages list --all succeeds without JSON errors
- [ ] #3 Audit other model files for the same pattern
<!-- AC:END -->
