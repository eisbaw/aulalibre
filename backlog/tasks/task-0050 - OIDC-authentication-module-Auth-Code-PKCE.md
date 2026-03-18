---
id: TASK-0050
title: OIDC authentication module (Auth Code + PKCE)
status: To Do
assignee: []
created_date: '2026-03-18 16:10'
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
- [ ] #1 PKCE code_verifier and code_challenge generation (S256)
- [ ] #2 Authorization URL construction with correct client_id, scope, redirect_uri, code_challenge
- [ ] #3 Local HTTP server to intercept OAuth redirect callback and extract auth code
- [ ] #4 Token exchange: POST to token endpoint with auth code and code_verifier
- [ ] #5 Support for both Level 2 (UniLogin) and Level 3 (MitID) auth
- [ ] #6 LoginData struct: access_token, refresh_token, expiration, error fields
- [ ] #7 Opens system browser for interactive login
<!-- AC:END -->
