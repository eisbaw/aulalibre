---
id: TASK-0116
title: >-
  aula-fuse: Log warnings on serde serialization failures instead of silent
  default
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:41'
labels:
  - aula-fuse
  - warning
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
unwrap_or_default() on serde_json::to_value() silently produces null JSON. At minimum log a warning when serialization fails so issues are visible.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Serialization failures produce a log warning with context
- [x] #2 No silent unwrap_or_default on serde serialization
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Replaced 14 silent unwrap_or_default() calls on serde_json::to_value() and serde_json::to_string_pretty() in aula-fuse/src/fs.rs with unwrap_or_else closures that log a warn\!() with context (resource type and operation) before falling back to Value::Null or String::new().

Affected resource types: posts, threads, events, notifications, albums, documents, presence statuses.

One non-serde unwrap_or_default (Option chain on line 408) was intentionally left unchanged.

Tests: 40/40 pass, clippy clean (-D warnings).
<!-- SECTION:FINAL_SUMMARY:END -->
