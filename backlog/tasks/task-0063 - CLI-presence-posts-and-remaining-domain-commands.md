---
id: TASK-0063
title: 'CLI presence, posts, and remaining domain commands'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:13'
updated_date: '2026-03-18 20:04'
labels:
  - rust
  - aula-cli
dependencies:
  - TASK-0059
  - TASK-0055
  - TASK-0056
  - TASK-0057
  - TASK-0058
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement CLI commands for remaining domains. Presence: 'aula presence status' (children's current state), 'aula presence schedule' (weekly schedule), 'aula presence update <child-id> --status <status>' (report sick/absent). Posts: 'aula posts list', 'aula posts show <id>', 'aula posts create --group <id> --body <text>'. Gallery: 'aula gallery albums', 'aula gallery show <album-id>'. Documents: 'aula documents list', 'aula documents show <id>'. Notifications: 'aula notifications list'. Search: 'aula search <query>'. Groups: 'aula groups list', 'aula groups show <id>'. Profile: 'aula profile show'.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Presence commands: status, schedule, update (sick/absent reporting)
- [x] #2 Post commands: list, show, create
- [x] #3 Gallery commands: list albums, show album
- [x] #4 Document commands: list, show
- [x] #5 Notification, search, groups, profile commands
- [x] #6 Consistent output formatting across all commands
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Implement presence commands (status, schedule, report-absence) with async handlers, build_session pattern, --json support
2. Implement posts commands (list, show) with human-readable output
3. Implement gallery commands (list, show) with human-readable output
4. Implement documents commands (list, show) with human-readable output
5. Implement notifications (list), search (query), groups (list, show, members), profile (me, show, children, institutions)
6. Update main.rs handle dispatch to pass json/env_override to all new commands
7. Run e2e tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented all 8 domain CLI commands with full async API integration:
- presence: status, registrations, schedule, report-status
- posts: list, show, create
- gallery: list albums, show album media
- documents: list secure docs, show (internal/external)
- notifications: list, delete-all, delete-child
- search: global search with counts
- groups: list (by context), show, members
- profile: me (login profiles), master-data

All commands follow the established pattern: build_session, --json flag, human-readable default output.
All e2e checks pass (439 tests, clippy clean, fmt clean).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented all remaining CLI domain commands, replacing stubs with full async API integration.

Changes:
- presence.rs: status (children state), registrations, schedule, report-status (sick/absent)
- posts.rs: list (with filtering), show (detail view), create
- gallery.rs: list albums, show album media
- documents.rs: list secure documents, show internal/external document details
- notifications.rs: list, delete-all, delete-child
- search.rs: global search with doc-type counts
- groups.rs: list by context, show details, list members
- profile.rs: me (login profiles with institution info), master-data
- main.rs: updated dispatch to pass json/env_override to all new async handlers

All commands follow the established pattern:
- build_session() with environment resolution
- --json flag for machine-readable JSON output
- Human-readable tabular/structured default output
- Proper error handling with exit codes

Tests: 439 passed, clippy clean, fmt clean.
<!-- SECTION:FINAL_SUMMARY:END -->
