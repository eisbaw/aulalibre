---
id: TASK-0045
title: Calendar and event model types
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:09'
updated_date: '2026-03-18 17:38'
labels:
  - rust
  - aula-api
  - models
dependencies:
  - TASK-0042
references:
  - data_models.md
  - domain_concepts.md
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Rust structs for the calendar domain: EventBaseClass, EventDetailsDto, TimeSlot, VacationDetailsDto, Lesson, EventGroup, EventProfile, EventResource, CalendarSyncConfiguration, DelegatedAccess, ImportantDate. Events have 5 classes (Basic, Series, Timeslot, Lesson, Unknown) and 14 event types. Timeslot events are used for parent-teacher meeting booking. Vacation registrations have request/response workflow. See data_models.md Models.Calendar namespace and domain_concepts.md Section 1.4.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 EventBaseClass, EventDetailsDto, EventSummary structs with serde Deserialize
- [x] #2 TimeSlot, Lesson, VacationDetailsDto, VacationRegistration structs
- [x] #3 EventGroup, EventProfile, EventResource, EventRecurrence structs
- [x] #4 CalendarSyncConfiguration, DelegatedAccess structs
- [x] #5 Request parameter types for event CRUD and responses
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create aula/aula-api/src/models/calendar.rs with all calendar model structs
2. Organize into sections: core event types, timeslot types, lesson types, repeating events, resources, delegates, sync config, important dates, vacation, request/response types
3. Register module in mod.rs
4. Run just e2e to verify compilation and tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All structs implemented in aula/aula-api/src/models/calendar.rs. 13 serde deserialization/serialization tests added covering core types. All 206 tests pass, clippy clean, fmt check passes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added calendar and event model types in aula-api/src/models/calendar.rs.

Covers the full Models.Calendar namespace from the decompiled Aula assemblies:
- Core event types: EventBaseClass, EventDetailsDto, EventSimpleDto with all base class fields
- Event profiles: EventProfile, EventProfileDetails, EventGroup, EventGroupWithRolesDto
- Timeslot booking: TimeslotEventDto, TimeSlot, TimeSlotAnswer, TimeSlotIndex and simplified variants
- Lessons: Lesson, LessonSimple, LessonParticipant, LessonBase
- Repeating events: RepeatingEventDto, CreateRepeatingEventRequest
- Resources: EventResource, EventResourceCategory, CreateEventResource
- Delegated access: DelegateAccesses, DelegateAccessesItem, DelegateAccessesInput
- Calendar sync: CalendarSynchronisationConfigurationItem, create/update requests
- Important dates: ImportantDateItem, ImportantDateItemInvitee, ImportantDateItemProfile
- Vacation: VacationRegistrationDetailsResultDto, VacationChildrenDto, VacationDayDto, VacationWeekResultDto
- Event CRUD: CreateBaseEventRequest, CreateSimpleEventRequest, CreateTimeslotEventRequest, CreateRepeatingEventRequest
- Event responses: RespondSimpleEventRequest, RespondTimeslotEventRequest, BlockTimeSlotRequest
- Query parameters: GetEventsParameters, GetEventsForInstitutionRequestModel
- Support types: BirthdayEventDto, AggregatedEventsGroupByType, ConflictEventItem, CalendarResourceConflict

13 unit tests verify serde roundtripping for key types. All 206 tests pass.
<!-- SECTION:FINAL_SUMMARY:END -->
