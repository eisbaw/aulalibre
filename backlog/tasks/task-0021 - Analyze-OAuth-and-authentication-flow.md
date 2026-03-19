---
id: TASK-0021
title: Analyze OAuth and authentication flow
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 14:06'
updated_date: '2026-03-19 05:34'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The manifest reveals a multi-step auth flow: LauncherActivity -> LoginActivity -> LoginWithPinActivity/OTPSelectionActivity, with WebAuthenticationCallbackActivity handling OAuth callbacks from app-private.aula.dk. CloudStorageAuthInterceptor handles OneDrive and Google OAuth redirects. Map the complete authentication flow including token handling.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Full authentication flow documented from launcher to authenticated state
- [x] #2 OAuth redirect URIs and callback handling mapped
- [x] #3 Cloud storage auth integration (OneDrive, Google) documented
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Comprehensive analysis of the complete Aula authentication system from decompiled C# source.

Changes:
- Rewrote auth_flow.md with 338 lines of new content verified against ILSpy decompilation output
- Documented full login routing from LauncherActivity through PIN/biometric to OIDC flow
- Mapped ClickedLoginButton flow including retry logic (RequestRefreshTokenAsync tries twice)
- Documented all OAuth redirect URIs: HTTPS callback (app-private.aula.dk), custom scheme (com.netcompany.aulanativeprivate), cloud storage paths (/googleoauth2redirect, //onedrive2redirect)
- Complete Google Drive OAuth config: 3 client IDs per platform, drive.readonly scope, accounts.google.com endpoints
- Complete OneDrive OAuth config: shared client ID 47984900-bb20-4659-9f0d-700f5ab91571, Microsoft Graph file/site scopes
- Cloud storage tokens are cached 2 minutes in-memory only (not persisted)
- Added certificate pinning public keys, session timeout values, cookie sync mechanism
- Added source file index mapping 20 key files to namespaces

Tests:
- E2E tests pass (78 passed, 0 failed)
<!-- SECTION:FINAL_SUMMARY:END -->
