---
id: TASK-0051
title: 'Token storage, refresh, and session management'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:10'
updated_date: '2026-03-18 18:26'
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
- [x] #1 File-based token storage in XDG_DATA_HOME/aula/ (or platform-appropriate location)
- [x] #2 Token serialization/deserialization (JSON)
- [x] #3 Automatic token refresh when expired (with configurable buffer before expiry)
- [x] #4 401 response triggers automatic token refresh and request retry
- [x] #5 Logout: clear stored tokens and hit logout endpoint
- [x] #6 Session keep-alive mechanism
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add chrono dependency for proper time handling with serde support
2. Create token_store.rs: TokenStore struct with load/save/clear for LoginData as JSON in secrets/ dir
3. Create session.rs: SessionManager that wraps AulaClient with auto-refresh, 401 retry, keep-alive, and logout
4. Add is_expired_with_buffer() to LoginData for configurable buffer
5. Add IO error variant to AulaError
6. Update lib.rs to expose new modules
7. Write comprehensive tests
8. Run just e2e
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented two new modules in aula-api:

- token_store.rs: File-based JSON token persistence using XDG_DATA_HOME/aula/ with 0600 permissions, atomic writes via temp+rename, and idempotent clear.
- session.rs: Session wrapper with proactive token refresh (configurable buffer), 401 retry on Unauthorized/InvalidAccessToken, keep-alive delegation, and logout (clears local tokens + hits auth backend logout endpoint).

Also added:
- is_expired_with_buffer() to LoginData for configurable expiry buffer (mirrors Conf.BufferOnTokenExpiration)
- IO error variant to AulaError
- dirs dependency for platform-appropriate data directory

All 337 tests pass, clippy and fmt clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added token storage, automatic refresh, and session management to aula-api.

Changes:
- New token_store.rs: TokenStore struct providing file-based JSON persistence of LoginData in XDG_DATA_HOME/aula/ (or custom path). Atomic writes (temp+rename), Unix 0600 permissions, idempotent clear.
- New session.rs: Session struct wrapping AulaClient + TokenStore. Proactive token refresh with configurable buffer before expiry. All API methods (get/post/put/delete) automatically retry once on 401/InvalidAccessToken after refreshing. Keep-alive delegation and logout (local clear + remote endpoint).
- Extended LoginData with is_expired_with_buffer(secs) for proactive refresh, mirroring Conf.BufferOnTokenExpiration from APK.
- Added AulaError::Io variant for file system errors.
- Added dirs dependency for platform-appropriate data directory resolution.

Tests:
- 22 new tests across token_store and session modules (roundtrip, permissions, clear, expiry, logout, etc.)
- All 337 tests pass, clippy clean, fmt clean.
<!-- SECTION:FINAL_SUMMARY:END -->
