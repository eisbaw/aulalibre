---
id: TASK-0007
title: Decompile DEX with baksmali for cross-reference
status: To Do
assignee: []
created_date: '2026-03-18 13:31'
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
- [ ] #1 baksmali produces smali output for all DEX files
- [ ] #2 Output stored in classes.dex.decompiled.baksmali/ directories
- [ ] #3 Smali output can be searched for string constants and method calls
<!-- AC:END -->
