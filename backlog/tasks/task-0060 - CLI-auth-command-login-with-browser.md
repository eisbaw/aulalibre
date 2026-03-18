---
id: TASK-0060
title: CLI auth command (login with browser)
status: To Do
assignee: []
created_date: '2026-03-18 16:12'
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
- [ ] #1 'aula auth login' opens browser, catches redirect, stores tokens
- [ ] #2 'aula auth login --level 3' for MitID step-up authentication
- [ ] #3 'aula auth logout' clears tokens and hits logout endpoint
- [ ] #4 'aula auth status' shows auth state, token expiry, user info
- [ ] #5 'aula auth refresh' forces token refresh
- [ ] #6 Prints clear instructions for the user during browser-based login flow
<!-- AC:END -->
