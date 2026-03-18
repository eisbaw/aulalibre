---
id: TASK-0066
title: Mock HTTP server for API integration tests
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:14'
updated_date: '2026-03-18 20:42'
labels:
  - rust
  - testing
dependencies:
  - TASK-0049
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create a mock HTTP server (using wiremock-rs or similar) that simulates the Aula API for integration testing without requiring real credentials. The mock server should: (1) serve canned JSON responses for each API endpoint, (2) validate request structure (correct URL paths, required headers like csrfp-token, correct HTTP methods), (3) simulate error conditions (401 unauthorized, maintenance mode, heavy load, session expired), (4) simulate the CSRF token cookie/header flow. This enables testing the full HTTP client -> service -> deserialization pipeline without network access.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Mock server with canned responses for key API endpoints (at least: profiles, messages, calendar, presence)
- [x] #2 Request validation: correct HTTP methods, URL paths, headers
- [x] #3 CSRF flow simulation: set Csrfp-Token cookie, validate csrfp-token header
- [x] #4 Error condition simulation: 401, maintenance, heavy load, session expired
- [x] #5 Helper functions to start/configure mock server in tests
- [x] #6 Mock server is reusable across integration test modules
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add wiremock to dev-dependencies in aula-api/Cargo.toml
2. Create aula-api/src/test_helpers.rs with mock server builder, CSRF flow simulation, and canned response helpers
3. Export test_helpers as #[cfg(test)] module from lib.rs
4. Create aula-api/tests/mock_api_tests.rs with integration tests:
   - Profiles: get_profiles_by_login with canned response + CSRF flow
   - Messaging: get_thread_list with canned response
   - Calendar: get_events with canned response
   - Presence: get_childrens_state with canned response
   - Error conditions: 401, maintenance (503), session expired (subCode 13), invalid token (subCode 9)
   - Request validation: correct HTTP methods, URL paths, csrfp-token header
5. Run just e2e to verify everything passes
6. Check all ACs and finalize
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implementation complete:
- Added wiremock 0.6 to dev-dependencies
- Added AulaClient::with_base_url() for pointing at mock server
- Added AulaClient::set_cookie() for simulating CSRF cookies
- Created tests/mock_api_tests.rs with 18 tests in 7 modules:
  - profiles: canned response deserialization
  - messaging: thread list deserialization
  - calendar: events with simplified canned response
  - presence: children state deserialization
  - request_validation: CSRF header, user-agent, accept, content-type, HTTP methods
  - csrf_flow: cookie-to-header echo, absence verification
  - error_conditions: 401, 503, subcode 7/9/13, backend error code
  - helpers_and_reuse: multi-service session reuse
- All 457 tests pass (439 unit + 18 mock)
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added wiremock-rs mock HTTP server infrastructure for integration testing the Aula API client.

Changes:
- aula-api/Cargo.toml: added wiremock 0.6 to dev-dependencies
- aula-api/src/client.rs: added `with_base_url()` constructor for pointing client at custom URLs (e.g. mock servers), and `set_cookie()` for simulating server-set cookies (e.g. CSRF tokens)
- aula-api/tests/mock_api_tests.rs: 18 integration tests across 7 modules covering:
  - Canned responses for profiles, messaging, calendar, presence endpoints
  - Request validation (HTTP methods, URL paths, csrfp-token header, user-agent, accept, content-type)
  - CSRF cookie-to-header flow simulation and absence verification
  - Error conditions (401 Unauthorized, 503 Maintenance, session expired subcode 13, invalid token subcode 9, user deactivated subcode 7, backend error codes)
  - Helper function reusability across service modules

Tests: just e2e passes (439 unit + 18 mock + 112 serde = 569 total)
<!-- SECTION:FINAL_SUMMARY:END -->
