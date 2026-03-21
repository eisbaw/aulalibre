---
id: TASK-0119
title: >-
  aula-fuse: Extract generic populate function to reduce ~600 lines of
  duplication
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:49'
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
- [x] #1 Common populate/insert pattern extracted into a generic function
- [x] #2 Per-resource-type logic is limited to mapping API response to inode entries
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Define a PopulateSpec struct capturing per-resource differences (cache_key, ttl, has_more check, resource_type)
2. Define a generic populate_with method on AulaFs that takes closures for API call and item insertion
3. Rewrite each populate_* to call populate_with with appropriate closures
4. Keep insert_* functions unchanged
5. Run cargo test and clippy to verify
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Extracted a generic `populate` method on `AulaFs` that handles the common plumbing shared by all 7 `populate_*` functions: cache check, API fetch dispatch, inode lock + clear_children, cache update on success, and error logging on failure.

Each resource type now only provides two closures to `populate`:
- `fetch`: builds API params, locks session, calls the API via block_on
- `process`: maps the API response into inode entries and optional pagination dirs

Also extracted `insert_page_dir` helper for the duplicated pagination directory insertion, and converted all `insert_*` methods from `&self` methods to associated functions since they only need `&mut InodeTable`.

Net reduction: 122 lines (-464, +342). All 40 existing tests pass. Clippy clean with -D warnings.
<!-- SECTION:FINAL_SUMMARY:END -->
