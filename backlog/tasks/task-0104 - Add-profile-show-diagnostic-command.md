---
id: TASK-0104
title: Add profile show diagnostic command
status: To Do
assignee: []
created_date: '2026-03-19 22:00'
labels:
  - rust-cli
  - feature
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a 'profile show' CLI command that dumps all institution profiles, groups, children, and their IDs from the getProfilesByLogin and getProfileContext responses. This helps diagnose which IDs the API returns and what context the CLI operates in.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 aula profile show displays guardian institution profile IDs, child institution profile IDs, institution codes, group memberships, and children names
- [ ] #2 aula profile show --json outputs raw getProfilesByLogin and getProfileContext responses
<!-- AC:END -->
