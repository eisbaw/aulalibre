---
id: TASK-0060
title: CLI auth command (login with browser)
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:12'
updated_date: '2026-03-18 19:30'
labels:
  - rust
  - aula-cli
  - auth
dependencies:
  - TASK-0059
  - TASK-0050
  - TASK-0051
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the 'aula auth login' command that performs interactive OIDC login. Opens the system browser for UniLogin/MitID authentication, runs a local HTTP server to catch the OAuth redirect, exchanges the code for tokens, and stores them. Also implement 'aula auth logout' (clear tokens, hit logout endpoint), 'aula auth status' (show current auth state, token expiry, auth level), and 'aula auth refresh' (force token refresh). This is the bootstrap command users run first before any other CLI operations.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 'aula auth login' opens browser, catches redirect, stores tokens
- [x] #2 'aula auth login --level 3' for MitID step-up authentication
- [x] #3 'aula auth logout' clears tokens and hits logout endpoint
- [x] #4 'aula auth status' shows auth state, token expiry, user info
- [x] #5 'aula auth refresh' forces token refresh
- [x] #6 Prints clear instructions for the user during browser-based login flow
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Make redirect_uri configurable in auth.rs (currently hardcoded to app-private.aula.dk)
2. Add open-browser utility (open/xdg-open) to aula-api or aula-cli
3. Implement local HTTP callback server in aula-cli auth command:
   - Bind to localhost:0 (random port)
   - Build authorize URL with redirect_uri=http://localhost:PORT/callback
   - Open browser with authorize URL
   - Wait for redirect callback with auth code
   - Exchange code for tokens
   - Save to TokenStore
4. Implement auth status: load TokenStore, display expiry and auth level
5. Implement auth logout: call session.logout()
6. Implement auth refresh: call session.refresh_token()
7. Update AuthCommand enum to match AC (--level flag, Refresh subcommand)
8. Make main() async (tokio) to support async auth operations
9. Run e2e tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented full auth command suite:
- login: local TCP server on random port, PKCE, browser open, callback wait with timeout
- logout: clears tokens + hits logout endpoint
- status: shows auth level, token expiry, time remaining
- refresh: forces token refresh via stored refresh token
- Made redirect_uri configurable in auth.rs (was hardcoded to app-private.aula.dk)
- Made exchange_code accept optional redirect_uri override
- Made main() async with #[tokio::main]
- All 439 tests pass, clippy clean, fmt clean
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented the full `aula auth` CLI command suite with four subcommands:

**auth login [--level 2|3] [--timeout N]**
- Starts local TCP server on 127.0.0.1:0 (random port)
- Generates PKCE challenge and OIDC state parameter
- Builds authorize URL with redirect_uri pointing to localhost callback
- Opens system browser via `open` crate (with fallback instructions)
- Prints clear user instructions including the authorize URL
- Waits for HTTP callback with configurable timeout (default 120s)
- Parses authorization code from callback, validates state
- Sends success/failure HTML response to browser
- Exchanges code for tokens via OIDC token endpoint
- Saves tokens to TokenStore with expiry info
- --level 3 triggers MitID step-up authentication

**auth status**
- Loads tokens from TokenStore
- Shows auth level, refresh token availability, expiry timestamp, time remaining
- Handles expired tokens and missing sessions gracefully

**auth logout**
- Calls Session.logout() which clears local tokens and hits logout endpoint

**auth refresh**
- Creates Session and calls refresh_token()
- Reports new expiry on success

**API changes (aula-api):**
- AuthorizeParams.redirect_uri is now Option<String> (was hardcoded)
- exchange_code() accepts optional redirect_uri parameter
- Both default to the mobile app URI when None

**CLI changes (aula-cli):**
- main() is now async (#[tokio::main]) to support auth operations
- Environment resolution passed to auth handler
- Added reqwest, open, url dependencies

Tests: 439 pass, clippy clean, fmt clean.
<!-- SECTION:FINAL_SUMMARY:END -->
