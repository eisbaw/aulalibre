---
id: TASK-0045
title: Calendar and event model types
status: To Do
assignee: []
created_date: '2026-03-18 16:09'
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
- [ ] #1 EventBaseClass, EventDetailsDto, EventSummary structs with serde Deserialize
- [ ] #2 TimeSlot, Lesson, VacationDetailsDto, VacationRegistration structs
- [ ] #3 EventGroup, EventProfile, EventResource, EventRecurrence structs
- [ ] #4 CalendarSyncConfiguration, DelegatedAccess structs
- [ ] #5 Request parameter types for event CRUD and responses
<!-- AC:END -->
