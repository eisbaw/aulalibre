---
id: TASK-0103
title: Fix CLI to send both guardian and child institution profile IDs
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 22:00'
updated_date: '2026-03-19 22:08'
labels:
  - rust-cli
  - bug
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The web frontend sends both guardian and child institution profile IDs for posts, calendar, and notifications. The CLI only sends one or the other, causing missing/stale data. Add an all_institution_profile_ids() method to Session that combines both, and update all commands to use it where appropriate.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Session has all_institution_profile_ids() method combining guardian + child IDs
- [x] #2 posts command sends both guardian and child IDs (matching web: institutionProfileIds[]={guardian}&institutionProfileIds[]={child})
- [x] #3 posts command includes parent=profile parameter (matching web HAR)
- [x] #4 calendar command sends both guardian and child IDs (matching web: instProfileIds: [guardian, child])
- [x] #5 notifications service sends activeChildrenIds[] and activeInstitutionCodes[] parameters (matching web HAR)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add all_institution_profile_ids() to Session (session.rs) - combines guardian + child IDs, deduped
2. Add institution_codes() to Session - extracts institution codes from children
3. Add parent field to GetPostApiParameters (models/posts.rs)
4. Add parent=profile to query builder (services/posts.rs)
5. Update posts CLI to use all_institution_profile_ids()
6. Update calendar CLI to use all_institution_profile_ids()
7. Update notifications service to accept and send activeChildrenIds[] and activeInstitutionCodes[]
8. Update notifications CLI to pass children IDs and institution codes
9. cargo build, cargo test, cargo clippy
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fix CLI to send both guardian and child institution profile IDs, matching the web frontend behavior verified via HAR captures.

Changes:
- Added `all_institution_profile_ids()` to Session: combines guardian + child IDs (deduped)
- Added `children_institution_codes()` to Session: extracts institution codes from children profiles
- Added `parent` field to `GetPostApiParameters` and posts query builder
- Posts command now sends `parent=profile` and both guardian+child IDs
- Calendar command now sends both guardian+child IDs (was child-only)
- Notifications service now accepts and sends `activeChildrenIds[]` and `activeInstitutionCodes[]` parameters
- Updated all callers and tests (unit, integration, e2e) for new signatures

Files changed:
- aula/aula-api/src/session.rs
- aula/aula-api/src/models/posts.rs
- aula/aula-api/src/services/posts.rs
- aula/aula-api/src/services/notifications.rs
- aula/aula-cli/src/commands/posts.rs
- aula/aula-cli/src/commands/calendar.rs
- aula/aula-cli/src/commands/notifications.rs
- aula/aula-api/tests/e2e_live_tests.rs
- aula/aula-api/tests/service_integration_tests.rs

Tests: `just e2e` passes (build + test + clippy + fmt-check)
<!-- SECTION:FINAL_SUMMARY:END -->
