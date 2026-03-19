---
id: TASK-0098
title: Fix ScanningStatus enum case sensitivity for gallery show
status: Done
assignee: []
created_date: '2026-03-19 17:10'
updated_date: '2026-03-19 18:18'
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
- [x] #1 gallery show <album_id> returns media items without deserialization error
- [x] #2 ScanningStatus enum handles both lowercase and PascalCase values
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added #[serde(rename_all = "camelCase")] to all 80+ enum types across calendar, common, documents, gallery, and notifications modules. Updated all test fixtures and inline tests to use camelCase values matching the real API. Three enums in common.rs with SCREAMING_SNAKE_CASE were left unchanged as they use integer-like naming.
<!-- SECTION:FINAL_SUMMARY:END -->
