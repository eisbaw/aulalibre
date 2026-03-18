---
id: TASK-0015
title: Analyze Kotlin Multiplatform native modules
status: To Do
assignee: []
created_date: '2026-03-18 13:59'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The base APK contains multiple Kotlin Multiplatform target directories (commonMain, nativeMain, linuxMain, androidNativeMain, jsAndWasmSharedMain, etc.) with linkdata/module files. These suggest the app uses KMP for cross-platform code sharing. Investigate what functionality is in these native modules vs the .NET layer.
<!-- SECTION:DESCRIPTION:END -->
