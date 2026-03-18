---
id: TASK-0007
title: Decompile DEX with baksmali for cross-reference
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:31'
updated_date: '2026-03-18 14:51'
labels: []
dependencies:
  - TASK-0003
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Use baksmali to produce smali disassembly of DEX files. Smali is lower-level than jadx output but more reliable for obfuscated code. Use as cross-reference when jadx output is unclear.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 baksmali produces smali output for all DEX files
- [x] #2 Output stored in classes.dex.decompiled.baksmali/ directories
- [x] #3 Smali output can be searched for string constants and method calls
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Download baksmali 3.0.9 standalone jar (matching apktool version)
2. Run baksmali on classes.dex -> classes.dex.decompiled.baksmali/
3. Run baksmali on classes2.dex -> classes2.dex.decompiled.baksmali/
4. Verify smali output: check file count, search for string constants and method calls
5. Ensure output dirs are gitignored
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Used apktool 2.12.1 (baksmali 3.0.9 bundled) to decode the base APK.
apktool d produces smali/ (classes.dex) and smali_classes2/ (classes2.dex) directories.
Renamed to classes.dex.decompiled.baksmali/ and classes2.dex.decompiled.baksmali/.

Results:
- classes.dex: 7229 smali files (android/androidx/kotlin/mono bootstrap)
- classes2.dex: 5163 smali files (678 CRC64 app ACW dirs + xamarin/microsoft/mono)
- String constants searchable via const-string
- Method calls searchable via invoke-* instructions
- Cleaned up apktool_out intermediate dir (only kept smali output)
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Decompiled both DEX files from the Aula APK using baksmali 3.0.9 (bundled in apktool 2.12.1).

Output:
- classes.dex.decompiled.baksmali/: 7229 smali files (Android/Xamarin bootstrap layer)
- classes2.dex.decompiled.baksmali/: 5163 smali files (678 CRC64 Android Callable Wrapper dirs + framework)

Smali output provides lower-level cross-reference for jadx decompilation (TASK-6). String constants (const-string) and method calls (invoke-*) are fully searchable with ripgrep. Output directories are gitignored under the *.extracted/ pattern.
<!-- SECTION:FINAL_SUMMARY:END -->
