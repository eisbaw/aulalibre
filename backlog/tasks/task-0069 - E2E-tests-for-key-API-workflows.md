---
id: TASK-0069
title: E2E tests for key API workflows
status: To Do
assignee: []
created_date: '2026-03-18 16:15'
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
- [ ] #1 E2E test: login, fetch profiles, verify profile data structure
- [ ] #2 E2E test: list threads, read a thread, verify message structure
- [ ] #3 E2E test: list today's calendar events
- [ ] #4 E2E test: get children's presence status
- [ ] #5 E2E test: list posts for a group
- [ ] #6 E2E test: get notifications
- [ ] #7 All E2E tests skip gracefully without auth token
- [ ] #8 E2E tests marked #[ignore], runnable with 'just e2e' recipe
<!-- AC:END -->
