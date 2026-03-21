---
id: TASK-0112
title: 'aula-fuse: Fix parent directory (..) resolution for nested page dirs'
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
readdir always sets .. to ROOT_INO regardless of depth. For page directories (2/, 3/, etc.), cd .. should go to the parent resource dir or parent page dir, not root. The PageDir.parent_inode field exists but is unused.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 .. in page directories resolves to the correct parent inode
- [ ] #2 cd .. from any depth navigates correctly back up the tree
<!-- AC:END -->
