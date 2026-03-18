---
id: TASK-0004
title: Inventory native libraries (.so files)
status: To Do
assignee: []
created_date: '2026-03-18 13:31'
labels: []
dependencies:
  - TASK-0001
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Catalog all .so native libraries, their architectures (arm64-v8a, armeabi-v7a, x86, etc.), and identify what they implement. Determine if any business logic lives in native code vs just being support libraries (SSL, React Native bridge, etc.).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All .so files listed with architecture and size
- [ ] #2 Each library identified by purpose (crypto, bridge, UI, etc.)
- [ ] #3 Business-logic-bearing native libs flagged for deeper analysis
- [ ] #4 Third-party native SDKs identified (e.g. Firebase, React Native)
<!-- AC:END -->
