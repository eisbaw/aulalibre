---
id: TASK-0054
title: Calendar API service
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:11'
updated_date: '2026-03-18 18:45'
labels:
  - rust
  - aula-api
  - service
dependencies:
  - TASK-0049
  - TASK-0045
references:
  - api_endpoints.md
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the calendar API service with 35+ methods from api_endpoints.md Section 3.5. Event operations: GetEvents, GetEventDetail, GetDailyAggregatedEvents, GetDailyGroupEventCount, GetEventForGroup, GetSchoolEvents, GetEventTypes, DeleteEvent. Event responses: RespondSimpleEvent, RespondTimeslotEvent. Timeslot management: EditTimeslotEvent, BlockTimeSlot, DeleteTimeSlot. Lesson: UpdateLessonEvent. Vacation: AddVacation, GetVacation, DeleteVacation, GetFutureVacationRequest, RespondToVacationRegistrationRequest. Calendar sync: CRUD for sync configurations and consent. Delegated access: Get/SetDelegatedAccesses. Feed: GetIsCalendarFeedEnabledForMunicipality, GetFeedConfigurationById. Other: GetBirthdaysForGroup/Institution, GetTopImportantDate, CheckConflictEventForAttendees.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Event CRUD: list, detail, daily aggregated, by group, school events, types, delete
- [x] #2 Event responses: respond to simple and timeslot events
- [x] #3 Timeslot and lesson management
- [x] #4 Vacation registration: add, get, delete, respond to requests
- [x] #5 Calendar sync configuration and consent management
- [x] #6 Delegated access, birthdays, important dates, conflict check
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create aula/aula-api/src/services/calendar.rs with all 37 CalendarWebService methods
2. Register calendar module in services/mod.rs
3. Group methods by AC: event CRUD, event responses, timeslot/lesson management, vacation, sync config, delegated access/misc
4. Follow messaging.rs pattern: module doc comment with endpoint table, typed response aliases, async functions using Session
5. Run just e2e to verify compilation and tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All 37 CalendarWebService methods implemented in calendar.rs.
10 serialization tests added covering all 6 AC areas.
just e2e passes: 372 tests, clippy clean, fmt clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented CalendarWebService with 37 async methods covering all calendar API operations from api_endpoints.md Section 3.5.

Changes:
- Added aula-api/src/services/calendar.rs with methods organized by AC area:
  - Event CRUD: get_events, get_event_detail, get_daily_aggregated_events, get_daily_group_event_count, get_event_for_group, get_school_events, get_event_types, get_event_types_for_calendar_feed, delete_event
  - Event responses: respond_simple_event, respond_timeslot_event
  - Timeslot/lesson: edit_timeslot_event, block_time_slot, delete_time_slot, update_lesson_event
  - Vacation: add_vacation, get_vacation, delete_vacation, get_future_vacation_request, get_vacation_request_response, respond_to_vacation_registration_request
  - Sync config: get/create/update/delete calendar sync configurations, get/update sync consent
  - Misc: get/set delegated accesses, get profiles with delegated accesses, get birthdays for group/institution, get_top_important_date, check_conflict_event_for_attendees, get_is_calendar_feed_enabled_for_municipality, get_feed_configuration_by_id
- Registered calendar module in services/mod.rs
- 10 unit tests verifying serialization of request/input types

Tests: nix-shell --run just e2e (372 tests pass, clippy clean, fmt clean)
<!-- SECTION:FINAL_SUMMARY:END -->
