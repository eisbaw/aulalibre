---
id: TASK-0067
title: Integration tests for API service modules
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:14'
updated_date: '2026-03-18 20:55'
labels:
  - rust
  - testing
dependencies:
  - TASK-0066
  - TASK-0052
  - TASK-0053
  - TASK-0054
  - TASK-0055
  - TASK-0056
  - TASK-0057
  - TASK-0058
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Write integration tests for each API service module using the mock HTTP server from TASK-66. Tests verify the full pipeline: AulaClient -> service method -> HTTP request -> mock response -> deserialized result. Each service module should have tests for: (1) successful operations, (2) error handling (API errors, network errors), (3) pagination where applicable, (4) correct request construction (URL, method, body). Priority services: messaging (most complex), calendar, presence, profiles.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Integration tests for profile/configuration service
- [x] #2 Integration tests for messaging service (thread CRUD, reply, folders)
- [x] #3 Integration tests for calendar service (events, timeslots, vacations)
- [x] #4 Integration tests for presence service (state, registration, schedule)
- [x] #5 Integration tests for remaining services (posts, gallery, documents, notifications, search)
- [x] #6 Error handling tests: 401 triggers refresh, maintenance mode, session expired
- [x] #7 All integration tests pass with cargo test
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add integration tests for profile/configuration service (AC #1)
   - get_profile_master_data, get_max_file_size, is_app_deprecated, keep_alive
2. Add integration tests for messaging service (AC #2)
   - reply_to_thread, start_new_thread, get_folders, delete_threads, auto-reply
3. Add integration tests for calendar service (AC #3)
   - get_event_detail, get_birthdays, add_vacation, get_future_vacation_request
4. Add integration tests for presence service (AC #4)
   - get_presence_registrations, get_presence_schedules
5. Add integration tests for remaining services (AC #5)
   - posts: get_posts, create_post, bookmark_post
   - gallery: get_albums, create_album, delete_media
   - documents: get_secure_documents, get_common_files, get_max_documents_per_export
   - notifications: get_notifications, delete_notifications
   - search: global_search, search_for_recipients
   - groups: get_group, get_memberships_light
   - health: is_alive
   - consent: get_consents, post_consents
   - widget: get_aula_token
6. Add error handling tests for 401 retry, maintenance mode, session expired (AC #6)
7. Run nix-shell --run just e2e to verify all tests pass (AC #7)
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented 79 wiremock-based integration tests across 18 test modules in service_integration_tests.rs.

Key implementation details:
- Reused mock_session/mock_client/aula_envelope helper pattern from TASK-66
- Added json_response() and fixture_response() convenience helpers
- Fixed 18 compilation errors due to incorrect function signatures (discovered actual struct shapes for presence, pickup, posts, messaging, comments, files services)
- Fixed 3 runtime failures: SecureDocumentExportStatus enum variants, RichTextWrapperDto nested struct for auto-reply
- All 79 tests pass in 0.44s
- Full e2e suite passes (build + all tests + clippy + fmt)
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added comprehensive wiremock-based integration tests for all aula-api service modules.

Changes:
- New file: aula/aula-api/tests/service_integration_tests.rs (79 tests, 18 modules)
- Covers: profile/config, messaging, calendar, presence, posts, gallery, documents, notifications, search, groups, health, consent, widgets, onboarding, push notifications, comments, files, error handling
- Each test verifies correct HTTP method, URL path, query params, request body serialization, and response deserialization
- Error handling tests cover 401 token refresh, 403 forbidden, 404 not found, 500 server error, malformed JSON, and network errors

Tests:
- All 79 new tests pass (0.44s)
- Full e2e suite passes: build + 648 total tests + clippy + fmt-check
<!-- SECTION:FINAL_SUMMARY:END -->
