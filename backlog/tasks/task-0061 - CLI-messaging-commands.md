---
id: TASK-0061
title: CLI messaging commands
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:13'
updated_date: '2026-03-18 19:36'
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
- [x] #1 'aula messages list' with --unread, --marked, --folder filters
- [x] #2 'aula messages read <id>' shows thread messages, marks as read
- [x] #3 'aula messages send --to <recipient> --subject <subj> --body <body>'
- [x] #4 'aula messages reply <id> --body <body>'
- [x] #5 'aula messages delete <id>' and folder management commands
- [x] #6 Table-formatted output with sender, subject, date, unread indicator
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Refactor messages.rs command enum: add --unread/--marked/--folder filters to List, add Reply subcommand, add Folders/Move subcommands
2. Change handle() signature to async, accept json flag and env_override to build Session
3. Extract session-building helper (shared with auth.rs pattern)
4. Wire each subcommand to real aula_api::services::messaging calls
5. Implement human-readable table output (sender, subject, date, unread indicator)
6. Implement JSON output path
7. Update main.rs dispatch to pass json flag and env_override, make call async
8. Run just e2e to verify compilation and tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Implemented all messaging CLI commands: list, read/show, send, reply, mark-read, delete, folders, move
- Added --unread, --marked, --folder filters to list command
- Human-readable table output with sender/subject/date/unread-indicator
- JSON output via --json flag
- Session helper builds authenticated session from token store
- HTML tag stripping for terminal display of message bodies
- All 439 tests pass, clippy clean, fmt clean
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented all messaging CLI commands by wiring the stub handlers in aula-cli to real aula-api service calls.

Commands implemented:
- `messages list` with --unread, --marked, --folder filters, --page pagination
- `messages read <id>` (aliased as show) displays thread messages with HTML-stripped text
- `messages send --to <ids> --subject <s> --body <b>` starts a new thread
- `messages reply <id> --body <b>` replies to existing thread
- `messages mark-read <id>` fetches latest message ID then sets last-read
- `messages delete <id>` deletes thread
- `messages folders` lists message folders
- `messages move <id> --folder <fid>` moves thread to folder

All commands support --json for raw API JSON output. Default output is table-formatted with sender, subject, date, and unread indicator (*). Session helper extracted in messages.rs follows same pattern as auth.rs.

Tests: 439 passed, clippy clean, fmt clean.
<!-- SECTION:FINAL_SUMMARY:END -->
