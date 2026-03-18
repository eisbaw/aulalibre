---
id: TASK-0068
title: 'E2E test bootstrap: manual login token capture'
status: To Do
assignee: []
created_date: '2026-03-18 16:15'
labels:
  - rust
  - testing
  - e2e
dependencies:
  - TASK-0060
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create an E2E test bootstrap command that a developer runs once manually to obtain a real Aula auth token for end-to-end testing. This is 'cargo run --bin aula-cli -- auth login --e2e-test-mode' which: (1) performs full browser-based OIDC login, (2) stores the token in a dedicated E2E test token file (e.g. tests/e2e/.auth-token), (3) the token file is gitignored. E2E tests load this token at runtime. Include clear documentation on how to set up E2E testing. This is a prerequisite for all E2E tests -- without a valid token, E2E tests skip gracefully.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 E2E bootstrap command stores tokens in tests/e2e/.auth-token (gitignored)
- [ ] #2 E2E test harness loads token from file, skips tests if not present
- [ ] #3 Documentation: README or doc comments explaining E2E setup process
- [ ] #4 Token refresh works in E2E mode (re-use refresh token)
- [ ] #5 Graceful skip with clear message when no E2E token is available
<!-- AC:END -->
