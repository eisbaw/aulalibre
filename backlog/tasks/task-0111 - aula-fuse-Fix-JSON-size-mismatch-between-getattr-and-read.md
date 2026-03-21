---
id: TASK-0111
title: 'aula-fuse: Fix JSON size mismatch between getattr and read'
status: To Do
assignee: []
created_date: '2026-03-21 06:19'
labels:
  - aula-fuse
  - critical
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ContentSource::Json stores a Value but getattr size is computed from serializing the original struct. read() re-serializes the Value, which may produce different output. This causes truncation or errors in programs that trust getattr size. Fix by serializing once at insertion time.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 JSON content serialized once and stored as pre-rendered string/bytes
- [ ] #2 getattr size matches actual read() output for all JSON files
- [ ] #3 ContentSource::Json variant removed or stores pre-serialized data
<!-- AC:END -->
