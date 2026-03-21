---
id: TASK-0114
title: 'aula-fuse: Fix strip_html checking text content instead of tag names'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:36'
labels:
  - aula-fuse
  - warning
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
strip_html checks result.ends_with("p") etc. after closing >, but result contains accumulated text, not tag names. Text ending in 'p' (e.g. 'trip') triggers spurious newlines. Track tag name during in_tag phase instead.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 strip_html correctly identifies HTML tags for newline insertion
- [x] #2 Text content like 'trip' does not trigger spurious newlines
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed strip_html to track tag names during the in_tag phase instead of checking result text content. The old code used result.ends_with("p") etc., which matched text like "trip" and inserted spurious newlines.

Changes:
- Added tag_name accumulator that captures the tag name inside < >
- Check tag_name (not result) on > to decide newline insertion
- Distinguish void elements (br) from paired block elements (p, div, li, h1-h6, tr) -- only insert newline on closing tags for paired elements
- Added 4 unit tests covering: basic stripping, no spurious newlines from text content, block tags producing newlines, and mixed content

Tests:
- cargo test -p aula-fuse: 40 passed
- cargo clippy -p aula-fuse -- -D warnings: clean
<!-- SECTION:FINAL_SUMMARY:END -->
