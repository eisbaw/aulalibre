---
id: TASK-0092
title: Implement Aula OIDC login flow in Rust
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 12:13'
updated_date: '2026-03-19 14:59'
labels: []
dependencies:
  - TASK-0091
references:
  - auth_flow.md
  - secrets/aula_login_20260319.har
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Build a Rust client that performs the Aula OIDC Authorization Code + PKCE login flow, matching the mobile app behavior documented in auth_flow.md. Uses the decompiled SimpleSAMLphp endpoints and client IDs. Must open a browser for the MitID/UniLogin step, listen for the redirect callback on https://app-private.aula.dk, and exchange the auth code for tokens. Ground truth from task-91 (Chrome DevTools login capture) provides reference for expected behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 OIDC Authorization Code + PKCE flow implemented against login.aula.dk endpoints
- [x] #2 Browser opens for MitID/UniLogin authentication; user pastes callback URL from app-redirect.aula.dk
- [x] #3 Access token, refresh token, and expiration captured and persisted to ~/.local/share/aula/
- [x] #4 Token refresh flow implemented using refresh_token grant (uses stored auth level for correct client_id)
- [x] #5 Authenticated API call succeeds using access_token query parameter (profiles.getprofilesbylogin)
- [x] #6 CSRF token extraction and header injection implemented for POST requests
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add Bearer token support to AulaClient (set_bearer_token method + Authorization header in decorate)
2. Session passes access token to client after load/refresh
3. Fix API version from 19 to 23 in session_util.rs and auth command
4. Build and run existing tests to verify nothing breaks
5. Test the actual login flow: `just run auth login --level 3`
6. If redirect_uri rejected, investigate alternatives (DNS override, custom scheme)
7. Make authenticated API call to verify Bearer token works
8. Test token refresh flow
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Key discovery: mobile app passes access_token as query parameter, NOT Bearer header. Decompiled OAuth2Request.GetAuthenticatedUrl() appends &access_token=<JWT> to every API URL.
- Bearer token in Authorization header caused HTTP 403 / error code 448.
- app-redirect.aula.dk wraps real callback URL in base64 returnUri parameter; CLI now decodes this automatically.
- Made WebResponseStatus.httpCode optional (real API omits it).
- Changed ProfilesByLoginResponse from Vec<Profile> to struct with profiles field to match real API shape.
- Token refresh was failing because SessionConfig defaulted to Level2 but token was obtained with Level3 (different client_id). Fixed to use auth_level from stored LoginData.
- User-Agent changed from 'AulaNative/2.15.4' to 'Android' to match real mobile app.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented the full Aula OIDC login flow in Rust, matching the mobile app's behavior as discovered through .NET decompilation.\n\nKey changes:\n- **Authentication method**: Changed from Bearer Authorization header to `access_token` query parameter, matching `OAuth2Request.GetAuthenticatedUrl()` from the decompiled app. Bearer returns HTTP 403/error 448.\n- **User-Agent**: Changed from `AulaNative/2.15.4` to `Android` to match real mobile app.\n- **app-redirect.aula.dk decoding**: CLI now decodes the base64 `returnUri` parameter from the intermediate redirect URL.\n- **Token refresh fix**: Uses auth level from stored LoginData (not config default) so Level 3 tokens refresh with the correct client_id.\n- **API response fixes**: Made `WebResponseStatus.httpCode` optional; changed `ProfilesByLoginResponse` to struct with `profiles` field.\n- **Dead code removal**: Removed unused `SessionConfig.auth_level` field.\n\nFiles changed:\n- `aula-api/src/client.rs` — access_token query param, User-Agent\n- `aula-api/src/session.rs` — token refresh auth level fix, removed dead field\n- `aula-api/src/response.rs` — optional httpCode\n- `aula-api/src/services/profiles.rs` — ProfilesByLoginResponse struct\n- `aula-cli/src/commands/auth.rs` — app-redirect base64 decoding with tests\n- `aula-cli/src/commands/profile.rs` — updated for new response type\n- `aula-cli/Cargo.toml` — added base64 dependency\n- Tests and fixtures updated throughout\n\nVerified end-to-end: OIDC login -> authenticated API call -> token refresh -> API call after refresh, all working against production Aula API. 535 tests pass.
<!-- SECTION:FINAL_SUMMARY:END -->
