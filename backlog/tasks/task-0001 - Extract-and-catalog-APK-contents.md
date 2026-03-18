---
id: TASK-0001
title: Extract and catalog APK contents
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:31'
updated_date: '2026-03-18 14:00'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Run apk_extract.sh on the Aula XAPK/APK to recursively extract all nested APKs. Catalog every extracted file with type, size, and path. This is the foundation for all subsequent analysis.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 apk_extract.sh runs successfully on the Aula APK
- [x] #2 All nested APKs extracted recursively
- [x] #3 extraction_manifest.txt lists every extracted file with metadata
- [x] #4 extraction_summary.txt shows counts by file type
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Run apk_extract.sh via nix-shell on the XAPK file
2. Verify extraction succeeded and nested APKs were found
3. Enhance extraction_manifest.txt with per-file metadata (type, size, path)
4. Enhance extraction_summary.txt with counts by file type
5. Create follow-up backlog tasks for interesting findings
6. Check each AC as completed
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Fixed 3 bugs in apk_extract.sh:
  1. log functions used tee (stdout), polluting find_apk_files return values; switched to stderr
  2. unzip missing -o flag for overwrite safety
  3. ((success_count++)) returns falsy when count=0 under set -e; added || true
- XAPK extracted successfully: 1 XAPK -> 4 nested APKs (base, config.en, config.mdpi, config.x86_64)
- 2270 total files, 2 DEX files, 12 native .so libraries, 4 AndroidManifest.xml files

- All 4 ACs checked: extraction runs, nested APKs found, manifest has per-file metadata, summary has type counts
- Key discovery: app is .NET MAUI/Xamarin (not pure Java/Kotlin). Main logic is in libassemblies blob, not DEX files
- Created follow-up tasks: TASK-14 through TASK-18 for .NET architecture, KMP modules, protobuf schemas, permissions, and assembly extraction
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Extracted and cataloged the Aula APK (com.netcompany.aulanativeprivate v2.15.4).

Changes:
- Fixed 3 bugs in apk_extract.sh: log functions polluting stdout via tee (broke find_apk_files), missing -o flag on unzip, and ((count++)) returning falsy under set -e when count=0
- Enhanced extraction to generate per-file manifest (size_bytes | mime_type | path) and file-type-count summary
- Ran extraction: 1 XAPK -> 4 nested APKs (base, config.en, config.mdpi, config.x86_64) -> 2270 files total

Key findings:
- App is .NET MAUI/Xamarin (libmonodroid.so, libmonosgen-2.0.so), NOT pure Java/Kotlin
- Main app logic is in libassemblies.x86_64.blob.so (38.8MB packed .NET assemblies)
- Contains Kotlin Multiplatform modules (commonMain, nativeMain, etc.) suggesting hybrid architecture
- Has protobuf schemas for analytics and messaging
- Uses Firebase C2DM, biometric auth, camera, phone dialing

Created 5 follow-up tasks (TASK-14 to TASK-18) for deeper investigation.
<!-- SECTION:FINAL_SUMMARY:END -->
