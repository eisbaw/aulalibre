---
id: TASK-0117
title: 'aula-fuse: Document Mutex-across-block_on safety invariant'
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
Holding Mutex<Session> across block_on() is safe because FUSE callbacks are on non-async threads, but this invariant is not documented. Add a comment explaining why this is safe and what would break it.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Safety invariant documented as comment near the Mutex/block_on usage
<!-- AC:END -->
