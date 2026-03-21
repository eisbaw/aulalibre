---
id: TASK-0114
title: 'aula-fuse: Fix strip_html checking text content instead of tag names'
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
strip_html checks result.ends_with("p") etc. after closing >, but result contains accumulated text, not tag names. Text ending in 'p' (e.g. 'trip') triggers spurious newlines. Track tag name during in_tag phase instead.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 strip_html correctly identifies HTML tags for newline insertion
- [ ] #2 Text content like 'trip' does not trigger spurious newlines
<!-- AC:END -->
