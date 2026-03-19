---
id: TASK-0100
title: 'Expand search to cover posts, events, and groups (findGeneric workaround)'
status: To Do
assignee: []
created_date: '2026-03-19 18:47'
labels:
  - rust-cli
  - investigation
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
search.findGeneric is broken server-side (HTTP 500). Currently we fall back to search.findProfiles which only returns profile results. Investigate whether combining multiple search endpoints (findProfiles, findGroups, findMessage) can approximate the full-text search that findGeneric was supposed to provide. Also check periodically whether findGeneric gets fixed in newer API versions.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Evaluate which additional search endpoints return useful results
- [ ] #2 Implement combined search if viable, or document why not
<!-- AC:END -->
