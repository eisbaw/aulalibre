---
id: TASK-0113
title: 'aula-fuse: Evict stale inode entries when cache expires'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:34'
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
- [x] #1 Stale inode entries removed when cache expires and resource list is re-fetched
- [x] #2 clear_children is used (remove dead_code allow)
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Evict stale inode entries when cache expires in aula-fuse.

Changes:
- All 7 populate_* functions in fs.rs now call `inodes.clear_children(parent_ino)` before re-inserting entries when the cache has expired and fresh data is fetched from the API. This ensures that resources deleted server-side are removed from the inode table.
- `remove_recursive` in inode_table.rs now also removes entries from the `parents` map, fixing a leak where parent mappings for evicted inodes would persist forever.
- Removed `#[allow(dead_code)]` from `clear_children` since it is now actively used.
- Added test `clear_children_removes_parents_and_entries` verifying that eviction cleans up entries, children, and parents maps recursively.

Tests:
- 36/36 aula-fuse tests pass
- cargo clippy clean (zero warnings)
<!-- SECTION:FINAL_SUMMARY:END -->
