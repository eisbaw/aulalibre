---
id: TASK-0097
title: Fix --profile flag name collision between global and groups command
status: To Do
assignee: []
created_date: '2026-03-19 17:10'
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
- [ ] #1 groups list --profile 99002 works without panic
- [ ] #2 Global --profile flag still works for profile selection
- [ ] #3 No clap name collision between flags
<!-- AC:END -->
