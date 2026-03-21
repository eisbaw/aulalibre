---
id: TASK-0116
title: >-
  aula-fuse: Log warnings on serde serialization failures instead of silent
  default
status: To Do
assignee: []
created_date: '2026-03-21 06:19'
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
- [ ] #1 Serialization failures produce a log warning with context
- [ ] #2 No silent unwrap_or_default on serde serialization
<!-- AC:END -->
