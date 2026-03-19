---
id: TASK-0082
title: Map AndroidX DataStore preference keys used by Aula
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-18 23:48'
updated_date: '2026-03-19 06:44'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
AndroidX DataStore Preferences uses protobuf to persist key-value pairs in .preferences_pb files. Identify what preference keys the Aula app stores (auth tokens, user settings, feature flags, etc.) by searching for DataStore/PreferencesKeys usage in the decompiled code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Identify all AndroidX DataStore preference keys in decompiled Java/Kotlin code
- [ ] #2 Identify all SharedPreferences keys used by both Java/Kotlin and .NET layers
- [ ] #3 Map each preference key to its purpose (auth, settings, feature flags, etc.)
- [ ] #4 Document findings in a structured analysis
- [ ] #5 Create follow-up tasks for interesting tangents discovered
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
