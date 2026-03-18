---
id: TASK-0006
title: Decompile main DEX files with jadx
status: To Do
assignee: []
created_date: '2026-03-18 13:31'
labels: []
dependencies:
  - TASK-0003
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Use jadx to decompile the primary DEX files containing com.netcompany.aula.* classes. Store output in structured directories. Assess decompilation quality and identify obfuscated vs readable code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 jadx produces Java source for main DEX files
- [ ] #2 Output stored in classes.dex.decompiled.jadx/ directories
- [ ] #3 Decompilation success rate documented (% of classes)
- [ ] #4 Obfuscation patterns identified (ProGuard, R8, etc.)
- [ ] #5 Key packages identified: API clients, models, auth, UI
<!-- AC:END -->
