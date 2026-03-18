---
id: TASK-0031
title: Map complete API endpoint catalog from Urls class
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 15:24'
updated_date: '2026-03-18 22:50'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Urls static class contains 100+ API endpoints organized by module (messages, calendar, presence, gallery, etc.). Extract the full URL catalog for API documentation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Update API version from 19 to 23 in client.rs defaults and all tests
- [x] #2 Redesign client URL construction for RPC-style ?method= routing
- [x] #3 Update ALL service modules to use correct ?method=module.action endpoints from Urls.cs
- [x] #4 Update mock API tests to use correct RPC-style paths
- [x] #5 Update e2e.rs to use API v23
- [x] #6 All tests pass (just e2e)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Change API version default from 19 to 23 in AulaClientConfig
2. Update client.rs url() method to handle ?method= query-string paths (the base_url ends with / and paths like ?method=messaging.getThreads need special handling - they go AFTER the base URL without path joining)
3. Systematically update every service module to use the correct RPC-style endpoint strings from Urls.cs
4. Update all tests (client.rs tests, mock_api_tests.rs, e2e.rs) to use v23 and new paths
5. Run just e2e to verify everything compiles and passes
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All 96 tests pass (18 mock_api + 78 service_integration). Clippy clean, fmt clean.
Full just e2e passes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Rewrote all aula-api service modules from RESTful path-style endpoints to RPC-style ?method=module.action endpoints, matching the Urls.cs source of truth from the decompiled Aula Android APK.

Changes:
- Updated API version from v19 to v23 across client.rs, session.rs, e2e.rs, and all tests
- Added dual-mode URL construction in client.rs: paths starting with ? are appended as query strings; others are joined as path segments
- Rewrote all 20 service modules (profiles, messaging, calendar, presence, gallery, notifications, consent, widget, groups, health, comments, posts, personal_reference, onboarding, additional_master_data, push_notifications, files, search, documents, configuration) to use correct RPC method names from Urls.cs
- Updated HTTP methods where needed: DELETE/PUT operations become POST in RPC style; path params become query params
- Updated all 18 mock_api_tests with RPC-style path + query_param matchers
- Updated all 78 service_integration_tests with correct RPC method names and HTTP methods
- Fixed presence.rs: 40+ endpoint rewrites including special cases (add_vacation uses calendar prefix, get_available_presence_statuses maps to presence.getPresenceStates)

Tests:
- 96 tests pass (18 mock_api + 78 service_integration)
- Clippy clean, cargo fmt clean
- Full just e2e passes
<!-- SECTION:FINAL_SUMMARY:END -->
