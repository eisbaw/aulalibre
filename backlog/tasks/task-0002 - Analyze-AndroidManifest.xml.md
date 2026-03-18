---
id: TASK-0002
title: Analyze AndroidManifest.xml
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:31'
updated_date: '2026-03-18 14:07'
labels: []
dependencies:
  - TASK-0001
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse the AndroidManifest.xml from the main APK to identify activities, services, receivers, permissions, intent filters, and content providers. This reveals the app's entry points and declared capabilities.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All activities listed with their intent filters
- [x] #2 All services and broadcast receivers documented
- [x] #3 Permissions catalogued (requested and declared)
- [x] #4 Content providers and authorities identified
- [x] #5 Main launcher activity identified
- [x] #6 Findings documented in milestone2_analysis.md
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Parse main AndroidManifest.xml from base APK using aapt/aapt2 dump
2. If binary XML, use apktool to decode it
3. Extract all activities with intent filters
4. Extract all services and broadcast receivers
5. Extract all permissions (requested and declared)
6. Extract content providers and authorities
7. Identify main launcher activity
8. Check config APK manifests for additional info
9. Document everything in milestone2_analysis.md
10. Create follow-up tasks for interesting findings
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Used apktool to decode binary AndroidManifest.xml from base APK
- Config APK manifests are minimal (no code, resources/libs only)
- Found 55 activities, 5/6 services, 7 receivers, 5 providers, 14 permissions
- All app classes use Xamarin CRC64 naming (crc64XXXX.ClassName)
- Identified CRC64 namespace groupings: auth (crc64727613c41f254141), calendar (crc647e759e71f16a7378), etc.
- Key domains: app-private.aula.dk (auth), *.ncaula.com (staging), *.aula.dk (prod)
- OAuth flows for both Aula login and cloud storage (OneDrive, Google)
- MonoRuntimeProvider initializes at priority 1999999999
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed AndroidManifest.xml from Aula APK v2.15.4 (com.netcompany.aulanativeprivate).

Findings documented in milestone2_analysis.md:
- 55 activities organized into 15 functional groups (auth, messaging, calendar, come/go attendance, posts, albums, secure documents, vacation, profiles, notifications, groups, cloud storage, settings, search, reporting)
- 6 services: AulaFirebaseMessagingService (push), KeepAliveService (background), plus Firebase/Google data transport
- 7 broadcast receivers: Firebase C2DM, MAUI Essentials (connectivity, battery, energy saver), AndroidX ProfileInstaller
- 5 content providers: FileProvider, MAUI Essentials FileProvider, MonoRuntimeProvider (.NET init at priority 1999999999), AndroidX Startup, Firebase Init
- 14 permissions requested including biometrics, camera, phone calls, notifications, Samsung tracking
- 1 custom permission declared (DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION, signature protection)
- Deep links: app-private.aula.dk (OAuth), custom scheme for OneDrive/Google OAuth redirects
- Queried domains: *.aula.dk (prod), *.ncaula.com (staging), Google Docs, YouTube
- All app classes use Xamarin CRC64 hash prefixes; identified 14 namespace groups by CRC64 hash

Created 3 follow-up tasks: TASK-19 (CRC64 hash mapping), TASK-20 (ncaula.com investigation), TASK-21 (OAuth flow analysis).
<!-- SECTION:FINAL_SUMMARY:END -->
