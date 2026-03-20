---
id: TASK-0103
title: Fix CLI to send both guardian and child institution profile IDs
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 22:00'
updated_date: '2026-03-20 12:23'
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
Send both guardian and child institution profile IDs matching web frontend HAR capture.

Changes:
- Added all_institution_profile_ids() and children_institution_codes() to Session
- Posts command uses combined IDs + parent=profile parameter
- Calendar command uses combined IDs instead of child-only
- Notifications service sends activeChildrenIds[] and activeInstitutionCodes[]
- Fixed 5 pre-existing clippy warnings

Verified: posts now show current REDACTED-INST data. All 680 tests pass, clippy clean, e2e green.
<!-- SECTION:FINAL_SUMMARY:END -->
