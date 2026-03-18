---
id: TASK-0049
title: HTTP client foundation with cookie and CSRF handling
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:09'
updated_date: '2026-03-18 18:12'
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
- [x] #1 AulaClient struct wrapping reqwest::Client with cookie store
- [x] #2 CSRF token middleware: extract Csrfp-Token from cookies, inject as csrfp-token header
- [x] #3 Environment configuration enum (Production, Preprod, Test, Dev) with correct hosts
- [x] #4 Base URL construction: https://{backend_host}/api/v{version}/
- [x] #5 Non-production basic auth support (aula-user:Aula-1337)
- [x] #6 Keep-alive support via POST /profiles/keepAlive
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create client.rs module with AulaClient struct
2. Add Environment enum with all 8 environments from auth_flow.md
3. Implement cookie-store-backed reqwest::Client
4. CSRF extraction from cookie jar + header injection
5. Base URL construction: https://{host}/api/v{version}/
6. Non-production basic auth support
7. Generic get/post/put/delete methods with AulaServiceResponse<T> deserialization
8. Keep-alive method (POST /profiles/keepAlive)
9. Register client module in lib.rs
10. Run e2e tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Created client.rs with AulaClient struct wrapping reqwest::Client + Arc<Jar> cookie store
- Environment enum covers all 8 environments from auth_flow.md
- CSRF token extracted from cookie jar via CookieStore trait, injected as csrfp-token header on every request
- Base URL: https://{host}/api/v{version}/
- Non-prod basic auth applied via reqwest RequestBuilder::basic_auth on each request
- Generic get/post/post_empty/put/delete methods with AulaServiceResponse<T> deserialization
- Response handler maps sub-codes to typed AulaError variants (InvalidAccessToken, SessionExpired, StepUpRequired, UserDeactivated)
- keep_alive() method calls POST profiles/keepAlive
- 26 unit tests covering all environments, CSRF extraction, URL construction, and response handling
- http crate added as dev-dependency for constructing test responses
- All 286 tests pass, clippy clean, fmt clean
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added HTTP client foundation for the Aula API in aula-api/src/client.rs.

Changes:
- AulaClient struct wrapping reqwest::Client with Arc<Jar> cookie store for automatic cookie persistence
- Environment enum (Production, Preprod, Hotfix, Test1/3, Dev1/3/11) with correct backend and auth hosts from APK EnvironmentFactory
- CSRF middleware: extracts Csrfp-Token from cookie jar, injects as csrfp-token header on every request via decorate() helper
- Base URL construction: https://{backend_host}/api/v{version}/ (default: Production, API v19)
- Non-production basic auth (aula-user:Aula-1337) applied per-request via reqwest basic_auth
- Generic request methods (get, post, post_empty, put, delete) that deserialize AulaServiceResponse<T> envelopes
- Response handler maps sub-codes to typed errors: InvalidToken->InvalidAccessToken, SessionExpired, StepUpRequired, UserDeactivated
- HTTP status shortcuts: 401->Unauthorized, 503->Maintenance
- keep_alive() method for POST /profiles/keepAlive session extension
- Re-exports AulaClient, AulaClientConfig, Environment from crate root
- http crate added as dev-dependency for test response construction

Tests:
- 26 new unit tests covering environment config, CSRF extraction, URL construction, and all response handler branches
- All 286 tests pass, clippy clean, fmt check clean
<!-- SECTION:FINAL_SUMMARY:END -->
