---
id: TASK-0062
title: CLI calendar commands
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:13'
updated_date: '2026-03-18 19:49'
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
- [x] #1 'aula calendar list' with --from, --to, --group filters
- [x] #2 'aula calendar show <id>' with full event details
- [x] #3 'aula calendar today' shortcut for today's events
- [x] #4 'aula calendar respond <id> --accept/--decline/--tentative'
- [x] #5 'aula calendar birthdays' with group/institution filter
- [x] #6 Table-formatted output with date, time, title, type columns
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Expand CalendarCommand enum: add Today, Week (alias for list with date range), Respond, Birthdays
2. Change handle() signature to async, accept json + env_override (match messages.rs pattern)
3. Add build_session helper (reuse same pattern as messages.rs)
4. Implement handle_list: parse --from/--to dates, call get_events, table output
5. Implement handle_show: call get_event_detail, print full details
6. Implement handle_today: shortcut calling handle_list with today..today+1
7. Implement handle_week: shortcut calling handle_list with today..today+7
8. Implement handle_respond: call respond_simple_event with --accept/--decline/--tentative
9. Implement handle_birthdays: call get_birthdays_for_group or get_birthdays_for_institution
10. Update main.rs to pass json and env to calendar::handle (make it async)
11. Run just e2e
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented all 6 calendar subcommands:
- list: calls get_events or get_event_for_group depending on --group flag
- show: calls get_event_detail with full detail rendering
- today: shortcut for list with today..tomorrow date range
- week: shortcut for list with today..+7 days date range
- respond: calls respond_simple_event with --accept/--decline/--tentative
- birthdays: calls get_birthdays_for_group or get_birthdays_for_institution

All commands support --json flag and --env override.
Updated main.rs to pass json/env_override to calendar handler (now async).
Added chrono dependency for date arithmetic.
All e2e checks pass: build, 439 tests, clippy clean, fmt clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented calendar CLI commands for aula-cli, replacing stubs with fully functional async handlers.

Changes:
- aula-cli/src/commands/calendar.rs: Rewrote from 33-line stub to full implementation with 6 subcommands (list, show, today, week, respond, birthdays). Each command calls the corresponding aula-api calendar service, supports --json output, and renders human-readable table/detail output by default.
- aula-cli/src/main.rs: Updated calendar dispatch to pass json and env_override flags, made handler async.
- aula-cli/Cargo.toml: Added chrono dependency for date arithmetic in today/week shortcuts.

Subcommands:
- list: --from/--to date range, --group for group-filtered events, --institution for profile filter
- show <id>: Full event details including description, invitees, attachments
- today: Shortcut for today only
- week: Shortcut for next 7 days
- respond <id>: --accept/--decline/--tentative with optional --profile
- birthdays: --group or --institution with date range

Tests: just e2e passes (build + 439 tests + clippy + fmt).
<!-- SECTION:FINAL_SUMMARY:END -->
