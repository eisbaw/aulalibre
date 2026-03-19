---
id: TASK-0102
title: Fix messages --all deserialization error on integer ID field
status: To Do
assignee: []
created_date: '2026-03-19 21:51'
labels:
  - rust-cli
  - bug
  - serde
dependencies: []
references:
  - aula/aula-api/src/models/messaging.rs
  - aula/aula-cli/src/commands/messages.rs
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
messages list --all fails on page 1 with: "invalid type: integer 764530177, expected a string at line 1 column 23606". A MessageThreadLatestMessage.id field (or similar String field) receives an integer from the API on some threads. Added a deserialize_optional_string_from_any helper but the error persists - there may be additional String fields in the messaging models that also receive integers. Need to dump the raw page 1 response JSON and identify all affected fields.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 messages list --all fetches all pages without deserialization errors
- [ ] #2 All messaging model String fields that can receive integers are handled
<!-- AC:END -->
