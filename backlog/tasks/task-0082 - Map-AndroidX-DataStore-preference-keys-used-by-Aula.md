---
id: TASK-0082
title: Map AndroidX DataStore preference keys used by Aula
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 23:48'
updated_date: '2026-03-19 06:46'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
AndroidX DataStore Preferences uses protobuf to persist key-value pairs in .preferences_pb files. Identify what preference keys the Aula app stores (auth tokens, user settings, feature flags, etc.) by searching for DataStore/PreferencesKeys usage in the decompiled code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Identify all AndroidX DataStore preference keys in decompiled Java/Kotlin code
- [x] #2 Identify all SharedPreferences keys used by both Java/Kotlin and .NET layers
- [x] #3 Map each preference key to its purpose (auth, settings, feature flags, etc.)
- [x] #4 Document findings in a structured analysis
- [x] #5 Create follow-up tasks for interesting tangents discovered
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Trace DataStore usage in Java/Kotlin DEX code (confirmed Firebase-only from TASK-78)
2. Trace all SharedPreferences consumers in both DEX files
3. Trace all .NET SecureStorage and Preferences usage
4. Map every preference key to its purpose and owner
5. Document findings in preference_storage_analysis.md
6. Create follow-up tasks for interesting findings
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
No new tangent tasks needed:
- PIN brute-force already captured in TASK-84
- Tink key analysis already in TASK-81
- SQLite encryption key rotation could be a follow-up but is low priority
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Mapped all local storage mechanisms and preference keys used by the Aula Android app across both Java/Kotlin and .NET layers.

Changes:
- Created `preference_storage_analysis.md` documenting ~30+ unique preference keys across 4 storage mechanisms
- AndroidX DataStore: confirmed Firebase-only (4+ keys for heartbeat tracking)
- Android SharedPreferences: 14+ keys used by Firebase Messaging, Firebase App ID, Google Sign-In, Tink, and AndroidX
- .NET SecureStorage (EncryptedSharedPreferences): 12 keys for auth tokens, PIN, encryption keys, portal role, biometrics, app state
- MAUI Preferences: 2 transient session timing keys (unencrypted, non-sensitive)

Security findings:
- All sensitive Aula data correctly uses EncryptedSharedPreferences via Plugin.SecureStorage
- PIN stored as plain string within encrypted storage (see TASK-84)
- SQLite encryption key generated once, never rotated

Tests: `just e2e` passes (453 unit + 78 integration + 1 doc test)
<!-- SECTION:FINAL_SUMMARY:END -->
