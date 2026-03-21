---
id: TASK-0115
title: 'aula-fuse: Use bytes not String for gallery media downloads'
status: To Do
assignee: []
created_date: '2026-03-21 06:19'
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
- [ ] #1 Media downloads use resp.bytes() not resp.text()
- [ ] #2 Binary content (images, videos) is not corrupted by UTF-8 conversion
<!-- AC:END -->
