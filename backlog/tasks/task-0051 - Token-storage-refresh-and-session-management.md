---
id: TASK-0051
title: 'Token storage, refresh, and session management'
status: To Do
assignee: []
created_date: '2026-03-18 16:10'
labels:
  - rust
  - aula-api
  - auth
dependencies:
  - TASK-0050
references:
  - auth_flow.md
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement secure token persistence and automatic refresh. Tokens (access_token, refresh_token, expiration) are stored locally (file-based for CLI, e.g. in XDG_DATA_HOME/aula/). Token refresh: POST to token endpoint with refresh_token grant type when access_token is expired (check expiration with configurable buffer like Conf.BufferOnTokenExpiration). Session keep-alive via periodic POST /profiles/keepAlive. Automatic retry on 401 with token refresh. Logout clears stored tokens. See auth_flow.md Sections 4-7.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 File-based token storage in XDG_DATA_HOME/aula/ (or platform-appropriate location)
- [ ] #2 Token serialization/deserialization (JSON)
- [ ] #3 Automatic token refresh when expired (with configurable buffer before expiry)
- [ ] #4 401 response triggers automatic token refresh and request retry
- [ ] #5 Logout: clear stored tokens and hit logout endpoint
- [ ] #6 Session keep-alive mechanism
<!-- AC:END -->
