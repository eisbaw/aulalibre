---
id: TASK-0021
title: Analyze OAuth and authentication flow
status: To Do
assignee: []
created_date: '2026-03-18 14:06'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The manifest reveals a multi-step auth flow: LauncherActivity -> LoginActivity -> LoginWithPinActivity/OTPSelectionActivity, with WebAuthenticationCallbackActivity handling OAuth callbacks from app-private.aula.dk. CloudStorageAuthInterceptor handles OneDrive and Google OAuth redirects. Map the complete authentication flow including token handling.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Full authentication flow documented from launcher to authenticated state
- [ ] #2 OAuth redirect URIs and callback handling mapped
- [ ] #3 Cloud storage auth integration (OneDrive, Google) documented
<!-- AC:END -->
