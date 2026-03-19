---
id: TASK-0088
title: Complete extraction and decompilation of ALL .NET assemblies from blob
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-19 06:47'
updated_date: '2026-03-19 07:13'
labels: []
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The 38MB libassemblies.x86_64.blob.so contains hundreds of .NET assemblies but only 30 have been extracted so far. Need to: (1) extract ALL assemblies from the blob using pyxamstore or manual parsing, (2) decompile all Aula-specific and key third-party DLLs with ILSpy, (3) catalog what was decompiled. This is foundational -- many analysis tasks depend on having the full decompiled source available.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ALL assemblies extracted from libassemblies blob (expect 200+ DLLs)
- [x] #2 All Aula-specific DLLs decompiled with ILSpy to C# source
- [x] #3 Key third-party DLLs decompiled (IdentityModel, Plugin.*, MonkeyCache, SQLite-net, etc.)
- [x] #4 Assembly inventory produced with name, size, and decompilation status
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Extraction complete:
- 363 XALZ entries found in blob
- 351 unique assemblies extracted (12 are locale variants)
- 0 extraction failures
- Assembly names reliably extracted by finding first .dll reference in PE metadata
- Categories: 3 aula-app, 21 third-party-key, 194 framework, 98 xamarin, 25 google-firebase, etc.

Decompilation complete:
- 24 key assemblies decompiled with ILSpy
- 3,796 total .cs source files generated
- All Aula-specific (AulaNative, AulaNative.Droid, AulaNative.Droid.Private) decompiled
- Key third-party libs decompiled: IdentityModel, Plugin.*, MonkeyCache, SQLite-net, AutoMapper, Newtonsoft.Json, etc.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Extracted ALL 363 .NET assemblies from the Xamarin blob (libassemblies.x86_64.blob.so), identifying 351 unique assemblies with 0 failures.

Changes:
- New extract_all_assemblies.py: parses XABA/XALZ blob format, decompresses via lz4, identifies assembly names from PE metadata
- New decompile_assemblies.sh: batch decompiles assemblies from the to-decompile list using ilspycmd
- New assembly_inventory.md: complete inventory of all 351 assemblies with name, size, category, and decompilation status
- Updated shell.nix: added python3Packages.lz4 for proper LZ4 decompression
- Updated .gitignore: added extracted_assemblies_all/

Key findings:
- 3 Aula-specific assemblies (AulaNative 4.1MB, AulaNative.Droid 2.3MB, AulaNative.Droid.Private 5KB)
- 21 key third-party libraries identified and decompiled
- 24 assemblies decompiled to C# with ILSpy (3,796 .cs files total)
- 194 framework, 98 Xamarin, 25 Google/Firebase assemblies cataloged

Extracted DLLs in extracted_assemblies_all/ and decompiled C# in decompiled_csharp/ (both gitignored).
<!-- SECTION:FINAL_SUMMARY:END -->
