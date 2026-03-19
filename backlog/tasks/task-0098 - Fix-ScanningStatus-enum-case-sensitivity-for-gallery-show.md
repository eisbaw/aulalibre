---
id: TASK-0098
title: Fix ScanningStatus enum case sensitivity for gallery show
status: To Do
assignee: []
created_date: '2026-03-19 17:10'
labels:
  - rust-cli
  - bug
  - serde
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
gallery show fails with deserialization error: unknown variant "bypassed", expected "Bypassed". The API returns lowercase scanning status values but the ScanningStatus enum expects PascalCase. Need to add #[serde(rename_all = "camelCase")] or aliases to the enum, similar to how other enums were fixed in TASK-0094.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 gallery show <album_id> returns media items without deserialization error
- [ ] #2 ScanningStatus enum handles both lowercase and PascalCase values
<!-- AC:END -->
