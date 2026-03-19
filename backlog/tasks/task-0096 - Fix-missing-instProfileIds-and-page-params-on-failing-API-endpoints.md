---
id: TASK-0096
title: Fix missing instProfileIds and page params on failing API endpoints
status: To Do
assignee: []
created_date: '2026-03-19 17:10'
labels:
  - rust-cli
  - api
dependencies:
  - TASK-0094
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Seven CLI commands return HTTP 400 code 40: messages folders, messages read, calendar list, presence status, presence registrations, presence schedule, documents list. The likely cause is missing mandatory query parameters (instProfileIds[], page, etc.) that the mobile app sends but our CLI doesn't. Need to: (1) add verbose request URL logging to identify exactly what's being sent, (2) compare with the decompiled app's parameter construction, (3) add the missing params. The session already calls getProfilesByLogin which returns institutionProfileIds - these need to be threaded through to the endpoint calls.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 messages read <thread_id> returns thread messages
- [ ] #2 messages folders returns folder list
- [ ] #3 calendar list returns events for today
- [ ] #4 presence status returns child attendance
- [ ] #5 documents list returns document list
- [ ] #6 All fixes verified against production API
<!-- AC:END -->
