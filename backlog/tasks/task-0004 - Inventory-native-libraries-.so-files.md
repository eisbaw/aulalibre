---
id: TASK-0004
title: Inventory native libraries (.so files)
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:31'
updated_date: '2026-03-18 14:23'
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
- [x] #1 All .so files listed with architecture and size
- [x] #2 Each library identified by purpose (crypto, bridge, UI, etc.)
- [x] #3 Business-logic-bearing native libs flagged for deeper analysis
- [x] #4 Third-party native SDKs identified (e.g. Firebase, React Native)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. List all .so files with size and architecture (readelf -h)
2. Classify each library by purpose using file, strings, nm, readelf
3. Identify .NET runtime vs platform vs third-party vs business logic
4. Flag any libraries containing business logic for deeper analysis
5. Document findings
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Analysis complete. All 12 .so files analyzed with readelf, nm, strings, and file.

Key findings:
- All libraries are x86_64 ELF64 shared objects (this APK split is config.x86_64)
- No business logic in native code - all .so files are infrastructure
- The .NET assemblies blob (38.8MB) contains the actual app logic as packed DLLs
- libxamarin-app.so contains 680 Java-to-.NET type mappings with CRC64 hashes
- Identified 3 Aula-specific DLLs: AulaNative.dll, AulaNative.Droid.dll, AulaNative.Droid.Private.dll
- Firebase messaging SDK present for push notifications
- SQLite used via SQLitePCLRaw wrapper
- SixLabors.ImageSharp for image processing
- IdentityModel.OidcClient for OAuth/OIDC authentication
- Plugin.Fingerprint for biometric auth
- Plugin.SecureStorage for secure credential storage
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Inventoried all 12 native .so files from config.x86_64.apk.

Findings:
- All 12 libraries are x86_64 ELF64 shared objects, totaling ~46 MB
- NO business logic in native code -- all are infrastructure/runtime
- 3 .NET runtime libs (Mono SGen runtime, Android bridge, marshal IL gen)
- 3 app metadata/store libs (packed assembly blob at 38MB, type mappings with 680 CRC64 entries, config stub)
- 4 .NET BCL native support libs (crypto, POSIX, compression, globalization)
- 2 third-party libs (SQLite engine, AndroidX DataStore counter)
- Business logic lives in packed .NET assemblies: AulaNative.dll, AulaNative.Droid.dll, AulaNative.Drod.Private.dll
- 16+ third-party SDKs identified via assembly names (Firebase, OIDC, Fingerprint, ImageSharp, MonkeyCache, etc.)

Output: native_library_inventory.md with full inventory, categorization, dependency graph, and third-party SDK list.
<!-- SECTION:FINAL_SUMMARY:END -->
