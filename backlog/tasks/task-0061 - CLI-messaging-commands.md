---
id: TASK-0061
title: CLI messaging commands
status: To Do
assignee: []
created_date: '2026-03-18 16:13'
labels:
  - rust
  - aula-cli
dependencies:
  - TASK-0059
  - TASK-0053
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement CLI commands for the messaging domain. 'aula messages list' (list threads, with filters: --unread, --marked, --folder), 'aula messages read <thread-id>' (show messages in thread, mark as read), 'aula messages send' (start new thread: --to, --subject, --body), 'aula messages reply <thread-id>' (reply with --body), 'aula messages delete <thread-id>', 'aula messages folders' (list folders), 'aula messages move <thread-id> --folder <folder-id>'. Output in table format by default, --json for raw API responses.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 'aula messages list' with --unread, --marked, --folder filters
- [ ] #2 'aula messages read <id>' shows thread messages, marks as read
- [ ] #3 'aula messages send --to <recipient> --subject <subj> --body <body>'
- [ ] #4 'aula messages reply <id> --body <body>'
- [ ] #5 'aula messages delete <id>' and folder management commands
- [ ] #6 Table-formatted output with sender, subject, date, unread indicator
<!-- AC:END -->
