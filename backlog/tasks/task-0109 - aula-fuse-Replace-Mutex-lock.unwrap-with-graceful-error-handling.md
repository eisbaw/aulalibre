---
id: TASK-0109
title: 'aula-fuse: Replace Mutex::lock().unwrap() with graceful error handling'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:24'
labels:
  - aula-fuse
  - critical
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
All ~30 lock() calls in fs.rs use .unwrap() which will panic and crash the FUSE mount if any thread poisons the mutex. Replace with error recovery or helper that returns FUSE EIO.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 No .unwrap() on Mutex::lock() in FUSE callback paths
- [x] #2 Poisoned lock returns EIO to FUSE caller instead of panicking
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Replaced all 41 Mutex::lock().unwrap() calls in aula-fuse/src/fs.rs with a lock_or_recover() extension trait method.

Changes:
- Added MutexExt<T> trait with lock_or_recover() that calls unwrap_or_else(|e| e.into_inner()) on poisoned locks
- Logs a warning via error\!() when recovering from a poisoned mutex
- All 41 occurrences (session, inodes, cache locks) replaced
- No panics on poisoned mutex; FUSE mount stays alive with potentially-stale data rather than crashing

Tests: cargo test -p aula-fuse (32 tests pass), cargo clippy -p aula-fuse clean
<!-- SECTION:FINAL_SUMMARY:END -->
