---
id: TASK-0041
title: Core error types and API response wrappers
status: To Do
assignee: []
created_date: '2026-03-18 16:07'
labels:
  - rust
  - aula-api
dependencies:
  - TASK-0040
references:
  - api_endpoints.md
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Define the error type hierarchy for the aula-api crate. The Aula API uses AulaServiceResponse<T> wrappers with WebResponseStatus codes, sub-codes (WebResponseStatusSubCodeConstants), and specific error conditions (maintenance, session expired, step-up needed, heavy load, user deactivated). See api_endpoints.md Section 6 for the full error handler list.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 AulaError enum covering: HTTP errors, auth errors (invalid token, session expired, step-up needed), API errors (maintenance, heavy load, user deactivated, not responding), serialization errors, network errors
- [ ] #2 AulaServiceResponse<T> and DataArrayResponse<T> generic response wrapper structs with serde Deserialize
- [ ] #3 WebResponseStatus enum matching API status codes
- [ ] #4 Error type implements std::error::Error and Display
- [ ] #5 Result<T> type alias for convenience
<!-- AC:END -->
