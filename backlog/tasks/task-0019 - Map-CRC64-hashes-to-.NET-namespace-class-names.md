---
id: TASK-0019
title: Map CRC64 hashes to .NET namespace/class names
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-18 14:06'
updated_date: '2026-03-19 05:22'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The AndroidManifest uses Xamarin CRC64 hashes as Java package names (e.g., crc64727613c41f254141.LauncherActivity). These map to actual .NET namespaces. Reverse the CRC64 hashes by examining the .NET assemblies or smali code to recover the real namespace hierarchy. This would reveal the actual app architecture.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 CRC64 hash to .NET namespace mapping table produced
- [ ] #2 At least the major namespace groups (auth, calendar, messaging, etc.) identified
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Extract CRC64 hashes from AndroidManifest.xml
2. Extract .NET type registrations from smali register() calls
3. Cross-reference hashes between manifest and smali
4. Categorize namespaces by functional domain
5. Generate mapping table as markdown
<!-- SECTION:PLAN:END -->
