---
id: TASK-0119
title: >-
  aula-fuse: Extract generic populate function to reduce ~600 lines of
  duplication
status: To Do
assignee: []
created_date: '2026-03-21 06:19'
labels:
  - aula-fuse
  - observation
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
populate_posts, populate_messages, populate_calendar etc. all follow the same pattern: check cache, lock session, call API, lock inodes, insert items, update cache, handle pagination. A generic populate function parameterized by API call and item insertion could reduce this to ~100 lines.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Common populate/insert pattern extracted into a generic function
- [ ] #2 Per-resource-type logic is limited to mapping API response to inode entries
<!-- AC:END -->
