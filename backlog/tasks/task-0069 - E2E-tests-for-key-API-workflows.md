---
id: TASK-0069
title: E2E tests for key API workflows
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:15'
updated_date: '2026-03-18 21:09'
labels:
  - rust
  - testing
  - e2e
dependencies:
  - TASK-0068
  - TASK-0067
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Write end-to-end tests that exercise the real Aula API with a valid auth token (from TASK-68 bootstrap). These tests run against the production API and verify that our Rust client correctly interacts with the real server. Tests should be read-only where possible to avoid side effects. Key workflows: (1) Login and fetch profiles, (2) List message threads and read a thread, (3) List today's calendar events, (4) Get children's presence status, (5) List posts for a group, (6) Get notifications. All E2E tests must skip gracefully if no auth token is available. Mark E2E tests with #[ignore] so they don't run in CI by default -- run explicitly with 'cargo test -- --ignored'.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 E2E test: login, fetch profiles, verify profile data structure
- [x] #2 E2E test: list threads, read a thread, verify message structure
- [x] #3 E2E test: list today's calendar events
- [x] #4 E2E test: get children's presence status
- [x] #5 E2E test: list posts for a group
- [x] #6 E2E test: get notifications
- [x] #7 All E2E tests skip gracefully without auth token
- [x] #8 E2E tests marked #[ignore], runnable with 'just e2e' recipe
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add E2E tests to aula/aula-api/tests/e2e_live_tests.rs for each AC:
   - e2e_get_profiles: get_profiles_by_login, verify Vec<Profile> non-empty
   - e2e_list_threads: get_thread_list with default args, verify structure
   - e2e_read_thread: if threads exist, read first thread messages
   - e2e_calendar_events: get_events with today date range
   - e2e_presence_status: get_childrens_state using profile IDs from login
   - e2e_list_posts: get_posts with default params
   - e2e_notifications: get_notifications
   - e2e_search: global_search with simple query
2. All tests use try_session() pattern, #[ignore], #[tokio::test]
3. All tests are read-only (GET only)
4. Run just e2e to verify compilation
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented 7 new E2E test functions in e2e_live_tests.rs:
- e2e_get_profiles_by_login (AC#1): fetches profiles, asserts non-empty with institution_profile
- e2e_list_threads_and_read (AC#2): lists threads, reads first thread messages if any
- e2e_calendar_events_today (AC#3): queries events for today date range using inst profile IDs
- e2e_presence_status (AC#4): queries children presence state
- e2e_list_posts (AC#5): fetches posts with limit=5
- e2e_get_notifications (AC#6): fetches notifications, asserts IDs present
- e2e_global_search (bonus): exercises global search endpoint

All tests use try_session() skip pattern (AC#7), all marked #[ignore] (AC#8).
Added date helper functions (today/tomorrow) using epoch arithmetic to avoid chrono dependency.
All tests are strictly read-only - only GET endpoints called.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added 7 E2E live tests exercising key read-only Aula API workflows.

Changes:
- aula/aula-api/tests/e2e_live_tests.rs: Added tests for profiles, messaging threads, calendar events, presence status, posts, notifications, and global search
- Each test creates a session via try_session(), calls a read-only API endpoint, and verifies the response structure deserializes correctly
- Added date helper functions (epoch arithmetic) to avoid adding chrono as a dependency
- All tests skip gracefully when no auth token is available
- All tests marked #[ignore], runnable via just e2e-live

Tests:
- nix-shell --run just e2e passes (build + 447 unit tests + lint + fmt)
- 10 E2E tests correctly shown as ignored in normal test runs
<!-- SECTION:FINAL_SUMMARY:END -->
