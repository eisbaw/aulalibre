---
id: TASK-0094
title: Investigate API session initialization required before data endpoints
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 15:50'
updated_date: '2026-03-19 17:00'
labels:
  - rust-cli
  - api
  - investigation
dependencies:
  - TASK-0093
references:
  - secrets/aula_login_20260319.har
  - >-
    decompiled_csharp/AulaNative/AulaNative.ServiceManagers/ProfileServiceManager.cs
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Most API endpoints (messages, calendar, notifications, posts, presence) return HTTP 400 (code 40) or HTTP 403 (code 10, subCode 23) when called after login. The mobile app calls getprofilesbylogin first which likely establishes server-side session state (PHPSESSID). Need to investigate whether: (1) the profile call sets up required session state, (2) specific headers or cookies are needed, (3) institutionProfileId must be passed as a parameter, or (4) there's an additional setup call required. Error code 40 suggests missing required parameters; code 10/subCode 23 suggests authorization denied. Compare with HAR capture from task-91 to see what the real app sends.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Root cause identified for HTTP 400 code 40 on messages/calendar/presence endpoints
- [x] #2 Root cause identified for HTTP 403 code 10 subCode 23 on notifications/posts endpoints
- [x] #3 Required session initialization sequence documented
- [x] #4 At least one data endpoint (messages or calendar) returning real data after fix
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Root cause: the API requires getProfilesByLogin + getProfileContext calls to establish a PHP session before data endpoints work. Added ensure_context_initialized() to Session. Fixed enum serde to use camelCase. Messages and notifications now return real data. Calendar/presence need endpoint-specific param fixes (separate tasks).
<!-- SECTION:FINAL_SUMMARY:END -->
