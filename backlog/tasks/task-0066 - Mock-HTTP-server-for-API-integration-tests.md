---
id: TASK-0066
title: Mock HTTP server for API integration tests
status: To Do
assignee: []
created_date: '2026-03-18 16:14'
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
- [ ] #1 Mock server with canned responses for key API endpoints (at least: profiles, messages, calendar, presence)
- [ ] #2 Request validation: correct HTTP methods, URL paths, headers
- [ ] #3 CSRF flow simulation: set Csrfp-Token cookie, validate csrfp-token header
- [ ] #4 Error condition simulation: 401, maintenance, heavy load, session expired
- [ ] #5 Helper functions to start/configure mock server in tests
- [ ] #6 Mock server is reusable across integration test modules
<!-- AC:END -->
