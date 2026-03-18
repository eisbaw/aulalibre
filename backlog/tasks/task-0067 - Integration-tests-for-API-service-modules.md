---
id: TASK-0067
title: Integration tests for API service modules
status: To Do
assignee: []
created_date: '2026-03-18 16:14'
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
- [ ] #1 Integration tests for profile/configuration service
- [ ] #2 Integration tests for messaging service (thread CRUD, reply, folders)
- [ ] #3 Integration tests for calendar service (events, timeslots, vacations)
- [ ] #4 Integration tests for presence service (state, registration, schedule)
- [ ] #5 Integration tests for remaining services (posts, gallery, documents, notifications, search)
- [ ] #6 Error handling tests: 401 triggers refresh, maintenance mode, session expired
- [ ] #7 All integration tests pass with cargo test
<!-- AC:END -->
