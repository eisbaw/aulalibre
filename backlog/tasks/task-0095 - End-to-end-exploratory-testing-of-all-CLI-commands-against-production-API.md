---
id: TASK-0095
title: End-to-end exploratory testing of all CLI commands against production API
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 15:53'
updated_date: '2026-03-19 17:11'
labels:
  - rust-cli
  - testing
  - exploratory
dependencies:
  - TASK-0093
  - TASK-0094
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Systematically test every CLI command against the production Aula API to map what works, what fails, and what needs fixing. This covers: messages (list/read/folders), calendar (list), notifications (list), posts (list), presence (status/registrations/schedule), groups (list), documents (list), gallery (list), search. For each command, document: the exact error or success, the raw API response shape, any missing parameters, and what the decompiled app sends differently. This is exploratory testing to build a complete picture of API compatibility.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Every CLI command tested against production API with results documented
- [x] #2 For each failing command: error code, raw response, and likely cause noted
- [x] #3 For each working command: output format and data correctness verified
- [x] #4 Missing or incorrect model mappings identified and filed as separate tasks
- [x] #5 Summary table of command status (works/fails/partially works) produced
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
E2E test results: 12 WORKS, 10 FAILS, 1 empty (legitimate). Root causes: (1) systemic HTTP 400 on 7 endpoints - missing instProfileIds or page params, (2) clap --profile flag name collision on groups, (3) ScanningStatus enum case sensitivity on gallery show, (4) search returns server 500.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Tested all 24 CLI commands against production. 12 work, 10 fail, 1 returns empty (legitimate). Filed 4 follow-up tasks: TASK-0096 (missing API params, high), TASK-0097 (--profile flag collision, medium), TASK-0098 (ScanningStatus enum case, low), TASK-0099 (search 500, low).
<!-- SECTION:FINAL_SUMMARY:END -->
