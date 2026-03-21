---
id: TASK-0115
title: 'aula-fuse: Use bytes not String for gallery media downloads'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:39'
labels:
  - aula-fuse
  - warning
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
LazyDownload uses resp.text() and stores as String, which corrupts binary media files (images, videos). Use resp.bytes() and store/return Vec<u8> instead.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Media downloads use resp.bytes() not resp.text()
- [x] #2 Binary content (images, videos) is not corrupted by UTF-8 conversion
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Changed lazy_download() to return Vec<u8> instead of String, using resp.bytes() instead of resp.text(). The read() handler now works with Vec<u8> throughout, converting Text content via as_bytes().to_vec() and passing Empty as an empty Vec. This prevents binary media files (images, videos) from being corrupted by UTF-8 string conversion.

Files changed:
- aula/aula-fuse/src/fs.rs: lazy_download return type and read() data flow

Tests: 40/40 pass, clippy clean.
<!-- SECTION:FINAL_SUMMARY:END -->
