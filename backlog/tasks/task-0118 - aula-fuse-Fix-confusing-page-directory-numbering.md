---
id: TASK-0118
title: 'aula-fuse: Fix confusing page directory numbering'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:44'
labels:
  - aula-fuse
  - warning
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Page 0 content lives in the resource dir, page 1 is in directory '2', page 2 in '3'. No directory '1' exists. Consider using page-2/, page-3/ naming or documenting the scheme clearly.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Page directory naming is intuitive and consistent
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Renamed pagination subdirectories from bare numbers ("2", "3", ...) to "page-2", "page-3", etc. across all four paginated resource types (Posts, Messages, Gallery, Documents) in aula-fuse/src/fs.rs.

The resource directory itself implicitly holds page 1 content. When more data exists, a "page-2/" subdirectory appears inside it, which in turn may contain "page-3/", and so on. The "page-N" prefix makes the purpose of these directories immediately obvious.

All 40 existing tests pass. Clippy clean.
<!-- SECTION:FINAL_SUMMARY:END -->
