---
id: TASK-0099
title: Investigate search endpoint HTTP 500 internal error
status: To Do
assignee: []
created_date: '2026-03-19 17:10'
labels:
  - rust-cli
  - investigation
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
search "menu" returns HTTP 500 "intern fejl" (internal server error). Could be request format issue, missing required fields, or wrong parameter encoding. Need to compare with the web frontend's search request format from the HAR capture or decompiled app.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Root cause of search 500 error identified
- [ ] #2 search command returns results or clear error if server-side issue
<!-- AC:END -->
