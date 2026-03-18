---
id: TASK-0062
title: CLI calendar commands
status: To Do
assignee: []
created_date: '2026-03-18 16:13'
labels:
  - rust
  - aula-cli
dependencies:
  - TASK-0059
  - TASK-0054
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement CLI commands for the calendar domain. 'aula calendar list' (list events, --from/--to date range, --group), 'aula calendar show <event-id>' (event details), 'aula calendar today' (today's events), 'aula calendar respond <event-id> --accept/--decline/--tentative', 'aula calendar birthdays --group <id>'. Output events in table with date, time, title, type, response status.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 'aula calendar list' with --from, --to, --group filters
- [ ] #2 'aula calendar show <id>' with full event details
- [ ] #3 'aula calendar today' shortcut for today's events
- [ ] #4 'aula calendar respond <id> --accept/--decline/--tentative'
- [ ] #5 'aula calendar birthdays' with group/institution filter
- [ ] #6 Table-formatted output with date, time, title, type columns
<!-- AC:END -->
