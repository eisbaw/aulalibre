---
id: TASK-0063
title: 'CLI presence, posts, and remaining domain commands'
status: To Do
assignee: []
created_date: '2026-03-18 16:13'
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
- [ ] #1 Presence commands: status, schedule, update (sick/absent reporting)
- [ ] #2 Post commands: list, show, create
- [ ] #3 Gallery commands: list albums, show album
- [ ] #4 Document commands: list, show
- [ ] #5 Notification, search, groups, profile commands
- [ ] #6 Consistent output formatting across all commands
<!-- AC:END -->
