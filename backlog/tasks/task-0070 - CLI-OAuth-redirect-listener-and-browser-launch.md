---
id: TASK-0070
title: CLI OAuth redirect listener and browser launch
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 18:18'
updated_date: '2026-03-19 19:17'
labels:
  - rust
  - aula-api
  - auth
  - cli
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the interactive CLI login flow that opens the system browser for OIDC auth and runs a local HTTP server to intercept the redirect callback. Builds on the auth module from TASK-50 which provides PKCE, URL building, and token exchange. This task covers: (1) local TCP listener on localhost that captures the redirect to https://app-private.aula.dk with the auth code, (2) opening the system browser to the authorize URL, (3) orchestrating the full flow: generate PKCE -> open browser -> wait for redirect -> exchange code -> return LoginData.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Local HTTP server listens for OAuth redirect callback
- [x] #2 Opens system browser to authorization URL
- [x] #3 Full login flow orchestration: PKCE -> browser -> redirect -> token exchange
- [x] #4 Timeout handling if user does not complete login
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added localhost OAuth callback server (callback_server.rs) and automated browser login flow. Default `aula-cli auth login` now starts a local HTTP server on a random port, opens the browser to the OIDC authorize URL with localhost redirect_uri, waits for the callback with auth code, and exchanges tokens. Original manual copy-paste flow preserved via `--manual` flag. Caveat: localhost redirect_uri may be rejected by the Aula OIDC provider if only app-private.aula.dk is registered -- --manual is the fallback. 10 unit tests for the callback server.
<!-- SECTION:FINAL_SUMMARY:END -->
