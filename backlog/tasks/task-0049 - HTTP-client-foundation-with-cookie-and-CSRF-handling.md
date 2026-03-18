---
id: TASK-0049
title: HTTP client foundation with cookie and CSRF handling
status: To Do
assignee: []
created_date: '2026-03-18 16:09'
labels:
  - rust
  - aula-api
  - http
dependencies:
  - TASK-0041
references:
  - auth_flow.md
  - api_endpoints.md
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Build the core HTTP client for the aula-api crate using reqwest with cookie store. The Aula API requires: (1) cookie jar persistence across requests, (2) CSRF token extraction from 'Csrfp-Token' cookie and injection as 'csrfp-token' header on every request, (3) base URL construction as https://{host}/api/v{VERSION}/, (4) common headers. The client should support configurable environments (production www.aula.dk, preprod, test, dev -- see auth_flow.md Section 1). Non-prod environments need basic auth (aula-user:Aula-1337).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 AulaClient struct wrapping reqwest::Client with cookie store
- [ ] #2 CSRF token middleware: extract Csrfp-Token from cookies, inject as csrfp-token header
- [ ] #3 Environment configuration enum (Production, Preprod, Test, Dev) with correct hosts
- [ ] #4 Base URL construction: https://{backend_host}/api/v{version}/
- [ ] #5 Non-production basic auth support (aula-user:Aula-1337)
- [ ] #6 Keep-alive support via POST /profiles/keepAlive
<!-- AC:END -->
