---
id: TASK-0050
title: OIDC authentication module (Auth Code + PKCE)
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:10'
updated_date: '2026-03-18 18:19'
labels:
  - rust
  - aula-api
  - auth
dependencies:
  - TASK-0049
references:
  - auth_flow.md
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the OIDC Authorization Code + PKCE flow for authenticating with Aula's SimpleSAMLphp-based OIDC provider at login.aula.dk. Two auth levels: Level 2 (UniLogin, client_id _742adb5e..., scope 'aula') and Level 3 (MitID, client_id _99949a54..., scope 'aula-sensitive'). The flow requires: (1) generate PKCE code_verifier/code_challenge, (2) open browser to authorize endpoint, (3) intercept redirect to https://app-private.aula.dk with auth code, (4) exchange code for tokens at token endpoint. For CLI usage, a local HTTP server listens for the redirect callback. See auth_flow.md for complete flow details.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 PKCE code_verifier and code_challenge generation (S256)
- [x] #2 Authorization URL construction with correct client_id, scope, redirect_uri, code_challenge
- [ ] #3 Local HTTP server to intercept OAuth redirect callback and extract auth code
- [x] #4 Token exchange: POST to token endpoint with auth code and code_verifier
- [x] #5 Support for both Level 2 (UniLogin) and Level 3 (MitID) auth
- [x] #6 LoginData struct: access_token, refresh_token, expiration, error fields
- [ ] #7 Opens system browser for interactive login
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add dependencies: sha2, base64, rand for PKCE; url crate for URL building; tokio for local server
2. Create src/auth.rs module with:
   - AuthLevel enum (Level2/UniLogin, Level3/MitID) with client_id/scope accessors
   - PkceChallenge struct: generate code_verifier (32 random bytes, base64url), derive code_challenge (SHA-256, base64url)
   - OidcConfig: authorize/token endpoints derived from Environment
   - authorize_url() builder: constructs full authorize URL with query params
   - TokenResponse struct: access_token, refresh_token, id_token, expires_in, token_type
   - exchange_code(): POST to token endpoint with auth code + code_verifier
   - RefreshRequest/response for token refresh
   - LoginData struct matching the APK fields
3. Add auth error variants to AulaError
4. Register mod auth in lib.rs, re-export key types
5. Add unit tests for PKCE generation, URL construction, serde
6. Run e2e tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented auth.rs with:
- AuthLevel enum with client_id/scope accessors and serde support
- PkceChallenge: generate() with CSPRNG, from_verifier_bytes() for testing, SHA-256 + base64url
- OidcEndpoints: derived from Environment, covers authorize/token/issuer URLs
- build_authorize_url(): full query param construction with response_type, client_id, scope, redirect_uri, code_challenge, code_challenge_method, state
- exchange_code(): POST form-encoded to token endpoint, handles OAuth error responses
- refresh_token(): POST with grant_type=refresh_token
- TokenResponse, TokenErrorResponse: serde types for token endpoint
- LoginData: mirrors APK LoginData with from_token_response(), is_expired(), error()
- extract_code_from_redirect(): parses redirect URL for code/state/error
- generate_state(): random base64url state parameter
- Added AulaError::Auth variant for OIDC errors
- Added base64, sha2, rand, url dependencies
- 31 unit tests covering all components

AC #3 (local HTTP server) and AC #7 (browser open) deferred: user instructions explicitly say no browser automation. extract_code_from_redirect provides the protocol-level code extraction; a local listener + browser open belong in a follow-up CLI integration task.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented OIDC Authorization Code + PKCE authentication module for the Aula platform in aula-api/src/auth.rs.

Changes:
- New auth.rs module with complete OIDC protocol implementation
- AuthLevel enum (Level2/UniLogin, Level3/MitID) with client_id and scope accessors
- PkceChallenge: S256 code_verifier/code_challenge generation per RFC 7636
- OidcEndpoints: authorize/token/issuer URLs derived from Environment
- build_authorize_url(): constructs authorization URL with all required query parameters
- exchange_code()/refresh_token(): async token endpoint interactions with error handling
- TokenResponse, TokenErrorResponse, LoginData structs matching APK data model
- extract_code_from_redirect(): parses redirect URL for authorization code with state verification
- AulaError::Auth variant for OIDC-specific errors
- Added base64, sha2, rand, url crate dependencies

Tests:
- 31 new unit tests covering PKCE generation, URL construction, code extraction, serde, LoginData lifecycle
- All 317 tests pass, clippy clean, fmt clean

Note: AC #3 (local HTTP server) and AC #7 (browser open) intentionally deferred to TASK-70 per instructions to focus on protocol implementation only.
<!-- SECTION:FINAL_SUMMARY:END -->
