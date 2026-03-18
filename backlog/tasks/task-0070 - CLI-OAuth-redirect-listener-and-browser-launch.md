---
id: TASK-0070
title: CLI OAuth redirect listener and browser launch
status: To Do
assignee: []
created_date: '2026-03-18 18:18'
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
- [ ] #1 Local HTTP server listens for OAuth redirect callback
- [ ] #2 Opens system browser to authorization URL
- [ ] #3 Full login flow orchestration: PKCE -> browser -> redirect -> token exchange
- [ ] #4 Timeout handling if user does not complete login
<!-- AC:END -->
