---
id: TASK-0003
title: Identify and catalog DEX files
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:31'
updated_date: '2026-03-18 14:14'
labels: []
dependencies:
  - TASK-0001
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Find all classes.dex and additional DEX files across extracted APKs. Determine which DEX files contain Aula business logic vs third-party libraries. Prioritize for decompilation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All DEX files listed with sizes and parent APK
- [x] #2 DEX files classified as app-code vs third-party
- [x] #3 Priority ranking for decompilation established
- [x] #4 Package namespaces extracted from each DEX (com.netcompany.aula.* etc.)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Find all DEX files across extracted APKs
2. Parse DEX headers directly (Python struct) to extract class names
3. Classify by package namespace into app-code vs third-party
4. Deep-dive into CRC64 classes (Aula app code)
5. Identify Xamarin/Mono runtime layer
6. Establish decompilation priority ranking
7. Write dex_catalog.md with full analysis
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Found 2 DEX files: classes.dex (8.2MB, 7229 classes) and classes2.dex (4.6MB, 5163 classes)
- No DEX files in config split APKs (en, mdpi, x86_64)
- Parsed DEX files directly with Python struct module (no jadx needed for listing)
- 12,392 total classes: 93.1% third-party, 5.5% Aula app code (CRC64), 3.5% Xamarin runtime
- 678 CRC64 app classes across 179 unique namespace hashes in classes2.dex
- All app code is in classes2.dex; classes.dex is entirely AndroidX/Google/third-party
- Confirmed this is a .NET MAUI app: real business logic is in libassemblies.x86_64.blob.so (38.8MB), not DEX
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Cataloged all DEX files in the Aula APK v2.15.4 XAPK package.

Findings:
- 2 DEX files in the main APK: classes.dex (8.2MB, 7,229 classes) and classes2.dex (4.6MB, 5,163 classes)
- No DEX files in config split APKs (en, mdpi, x86_64)
- 12,392 total classes classified into 15 categories
- 93.1% third-party (AndroidX, Google/Firebase, Kotlin, Glide, etc.)
- 5.5% Aula app code (678 CRC64-namespaced classes across 179 unique namespace hashes)
- 3.5% Xamarin/Mono runtime bridge (438 classes)
- All app code resides in classes2.dex; classes.dex is entirely third-party
- Mapped CRC64 class groups to app features: messaging, calendar, media, secure documents, presence/ComeGo, profiles, notifications, absence/vacation, groups, search, auth

Key architectural finding: DEX files contain only the Android bootstrap layer. The real business logic is in packed .NET assemblies (libassemblies.x86_64.blob.so, 38.8MB — 3x the DEX size).

Decompilation priority: (1) CRC64 app classes for screen/activity mapping, (2) Xamarin runtime for bridge understanding, (3) third-party libs not needed.

Output: dex_catalog.md
Created follow-up tasks: TASK-0022 (CRC64 hash reversal), TASK-0023 (TLS cert handling), TASK-0024 (jadx decompilation)
<!-- SECTION:FINAL_SUMMARY:END -->
