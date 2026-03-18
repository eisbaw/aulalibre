---
id: TASK-0041
title: Core error types and API response wrappers
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:07'
updated_date: '2026-03-18 17:12'
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
- [x] #1 AulaError enum covering: HTTP errors, auth errors (invalid token, session expired, step-up needed), API errors (maintenance, heavy load, user deactivated, not responding), serialization errors, network errors
- [x] #2 AulaServiceResponse<T> and DataArrayResponse<T> generic response wrapper structs with serde Deserialize
- [x] #3 WebResponseStatus enum matching API status codes
- [x] #4 Error type implements std::error::Error and Display
- [x] #5 Result<T> type alias for convenience
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create error.rs with AulaError enum using thiserror (HTTP, auth, API, serialization, network variants)
2. Create response.rs with WebResponseStatus, AulaServiceResponse<T>, DataArrayResponse<T>, WebResponseStatusSubCode constants
3. Add Result<T> type alias in error.rs
4. Wire modules into lib.rs with pub mod + re-exports
5. Run just e2e to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented error.rs and response.rs modules.
- AulaError enum with 12 variants covering all 13 error handlers (GenericErrorHandler maps to Api variant)
- WebResponseStatus, WebResponseStatusSubCode (19 known sub-codes with from_code conversion)
- AulaServiceResponse<T>, DataArrayResponse<T>, AulaErrorResponse<T> with serde Deserialize
- Result<T> type alias
- 14 unit tests all passing, clippy clean, fmt clean
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added core error types and API response wrappers to the aula-api crate.

New files:
- error.rs: AulaError enum (12 variants via thiserror) covering HTTP, auth, API, serialization, and network errors. Includes Result<T> type alias.
- response.rs: WebResponseStatus, WebResponseStatusSubCode (19 known sub-codes), AulaServiceResponse<T>, DataArrayResponse<T>, AulaErrorResponse<T> — all with serde Deserialize.
- lib.rs updated with module declarations and re-exports.

All types derived from reverse-engineered Models.Web namespace and the 13 error handler classes in the decompiled APK.

14 unit tests covering serialization, error trait impl, type conversions, and minimal/full JSON payloads.
<!-- SECTION:FINAL_SUMMARY:END -->
