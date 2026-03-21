---
id: TASK-0112
title: 'aula-fuse: Fix parent directory (..) resolution for nested page dirs'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:31'
labels:
  - aula-fuse
  - critical
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
readdir always sets .. to ROOT_INO regardless of depth. For page directories (2/, 3/, etc.), cd .. should go to the parent resource dir or parent page dir, not root. The PageDir.parent_inode field exists but is unused.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 .. in page directories resolves to the correct parent inode
- [x] #2 cd .. from any depth navigates correctly back up the tree
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed parent directory (..) resolution in aula-fuse readdir.

Changes:
- Added `parents: HashMap<u64, u64>` to InodeTable tracking child-to-parent relationships
- Populated parent map in `InodeTable::new()` for resource dirs and in `insert()` for all entries
- Added `parent_of(ino)` method returning the correct parent inode (ROOT_INO for root itself)
- Replaced hardcoded `ROOT_INO` match in `fs.rs::readdir` with `inodes.parent_of(ino)`
- Removed unused ROOT_INO import from fs.rs

Tests:
- Added 3 unit tests: parent_of_root_is_root, parent_of_resource_dir_is_root, parent_of_nested_entries
- All 35 tests pass, clippy clean with -D warnings
<!-- SECTION:FINAL_SUMMARY:END -->
