---
id: TASK-0003
title: Identify and catalog DEX files
status: To Do
assignee: []
created_date: '2026-03-18 13:31'
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
- [ ] #1 All DEX files listed with sizes and parent APK
- [ ] #2 DEX files classified as app-code vs third-party
- [ ] #3 Priority ranking for decompilation established
- [ ] #4 Package namespaces extracted from each DEX (com.netcompany.aula.* etc.)
<!-- AC:END -->
