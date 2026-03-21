---
id: TASK-0113
title: 'aula-fuse: Evict stale inode entries when cache expires'
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
Cache uses TTL but inode table never evicts. If a resource is deleted server-side, its inode persists forever. clear_children exists but is dead code. Call clear_children before re-populating when cache expires.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Stale inode entries removed when cache expires and resource list is re-fetched
- [ ] #2 clear_children is used (remove dead_code allow)
<!-- AC:END -->
