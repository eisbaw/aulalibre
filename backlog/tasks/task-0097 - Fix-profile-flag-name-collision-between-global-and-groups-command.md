---
id: TASK-0097
title: Fix --profile flag name collision between global and groups command
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 17:10'
updated_date: '2026-03-20 21:49'
labels:
  - rust-cli
  - bug
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The global --profile flag (Option String, institution profile selector) name-collides with groups list's local --profile flag (i64, institution profile ID). Clap resolves to the global String type and panics with "Could not downcast to String, need to downcast to i64". Need to rename one of them - either global to --active-profile or groups' to --inst-profile.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Global --profile flag still works for profile selection
- [x] #2 No clap name collision between flags
- [ ] #3 groups list --inst-profile <ID> works without panic
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Renamed --profile flag to --inst-profile in groups command to avoid collision with global --profile flag. Updated help text accordingly.
<!-- SECTION:FINAL_SUMMARY:END -->
