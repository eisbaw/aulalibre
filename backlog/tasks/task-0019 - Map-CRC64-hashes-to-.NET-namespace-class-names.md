---
id: TASK-0019
title: Map CRC64 hashes to .NET namespace/class names
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 14:06'
updated_date: '2026-03-19 05:23'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The AndroidManifest uses Xamarin CRC64 hashes as Java package names (e.g., crc64727613c41f254141.LauncherActivity). These map to actual .NET namespaces. Reverse the CRC64 hashes by examining the .NET assemblies or smali code to recover the real namespace hierarchy. This would reveal the actual app architecture.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 CRC64 hash to .NET namespace mapping table produced
- [x] #2 At least the major namespace groups (auth, calendar, messaging, etc.) identified
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Extract CRC64 hashes from AndroidManifest.xml
2. Extract .NET type registrations from smali register() calls
3. Cross-reference hashes between manifest and smali
4. Categorize namespaces by functional domain
5. Generate mapping table as markdown
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Mapped all 179 Xamarin CRC64 hashed Java package names back to their original .NET namespaces by extracting mono.android.Runtime.register() calls from ACW smali code.

Key results:
- 179 unique CRC64 hashes mapped to .NET namespaces (677 type registrations)
- All 72 AndroidManifest.xml hashes matched (100%), plus 107 smali-only hashes
- Namespaces categorized into 13 functional domains: Authentication, Calendar, Messaging, Come&Go (attendance), Documents, Albums/Media, User Profile, Notifications, Groups, Search, Settings, Onboarding, and more
- Mapping reveals the full app architecture: AulaNative.Droid.Activities.* for screens, AulaNative.Droid.CustomViews.* for UI components, AulaNative.Droid.CrossPlatformServices.* for platform abstractions
- Also identified framework namespaces: Microsoft.Maui.*, AndroidX.Browser.*, Android.Gms.* etc.

Output: crc64_namespace_mapping.md at repo root
Script: analysis/crc64_mapper.py (gitignored with analysis/)
<!-- SECTION:FINAL_SUMMARY:END -->
