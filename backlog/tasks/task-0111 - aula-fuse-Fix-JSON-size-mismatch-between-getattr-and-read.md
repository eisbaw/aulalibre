---
id: TASK-0111
title: 'aula-fuse: Fix JSON size mismatch between getattr and read'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:28'
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
- [x] #1 JSON content serialized once and stored as pre-rendered string/bytes
- [x] #2 getattr size matches actual read() output for all JSON files
- [x] #3 ContentSource::Json variant removed or stores pre-serialized data
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed JSON size mismatch between getattr and read in aula-fuse.

Root cause: ContentSource::Json stored a serde_json::Value, but the inode size was computed by serializing the original struct. read() then re-serialized the Value, which could produce different output (different key ordering, whitespace). This meant getattr reported one size but read returned different-length data, causing truncation in programs that trust getattr.

Fix: All 7 metadata.json insertion sites now store the pre-rendered JSON string as ContentSource::Text(json), reusing the same string that was already serialized to compute the size. The ContentSource::Json variant and its read() match arm have been removed entirely.

Files changed:
- aula/aula-fuse/src/inode_table.rs: Removed Json variant from ContentSource enum
- aula/aula-fuse/src/fs.rs: Changed 7 ContentSource::Json(...) to ContentSource::Text(json), removed Json match arm in read()

Tests: 32 passed, clippy clean.
<!-- SECTION:FINAL_SUMMARY:END -->
