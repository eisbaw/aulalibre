---
id: TASK-0120
title: 'aula-fuse: Clean up minor code issues (dead code, missing open(), env parsing)'
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
Minor issues from MPED review: (1) InodeEntry::ResourceItem.name duplicates children HashMap key, (2) ContentSource::Empty is never constructed, (3) No open() validation, (4) parse_environment should use clap ValueEnum with hard error on typos instead of silent fallback to production.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Remove ContentSource::Empty or use it
- [ ] #2 parse_environment replaced with clap ValueEnum derive
- [ ] #3 Unknown --env value is a hard error, not silent fallback to production
<!-- AC:END -->
