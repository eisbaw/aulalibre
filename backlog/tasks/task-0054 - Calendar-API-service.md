---
id: TASK-0054
title: Calendar API service
status: To Do
assignee: []
created_date: '2026-03-18 16:11'
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
- [ ] #1 Event CRUD: list, detail, daily aggregated, by group, school events, types, delete
- [ ] #2 Event responses: respond to simple and timeslot events
- [ ] #3 Timeslot and lesson management
- [ ] #4 Vacation registration: add, get, delete, respond to requests
- [ ] #5 Calendar sync configuration and consent management
- [ ] #6 Delegated access, birthdays, important dates, conflict check
<!-- AC:END -->
