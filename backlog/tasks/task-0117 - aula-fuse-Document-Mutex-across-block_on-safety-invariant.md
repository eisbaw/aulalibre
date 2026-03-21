---
id: TASK-0117
title: 'aula-fuse: Document Mutex-across-block_on safety invariant'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:43'
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
- [x] #1 Safety invariant documented as comment near the Mutex/block_on usage
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added module-level doc comment in aula-fuse/src/fs.rs documenting the safety invariant for holding Mutex<Session> across Handle::block_on() calls.

The comment explains:
- Why safe: fuser dispatches callbacks on dedicated non-async threads
- What breaks it: calling from an async task (panic or deadlock)
- Why the serialization is acceptable (single-user mount)

No functional changes. Clippy passes clean.
<!-- SECTION:FINAL_SUMMARY:END -->
