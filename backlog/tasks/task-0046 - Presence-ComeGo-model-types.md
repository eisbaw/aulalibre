---
id: TASK-0046
title: Presence (ComeGo) model types
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:09'
updated_date: '2026-03-18 17:46'
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
Rust structs for the presence/attendance domain: PresenceRegistration, ChildStatus, PresenceSchedule, PresenceTemplate, PickupResponsible, OpeningHours, ClosedDay, SleepInterval, SpareTimeActivity, Location, VacationAnnouncement, PresenceModuleSettings, DailyOverview, PresenceFilter. ComeGo is a major subsystem with 40+ API methods and its own configuration system. See data_models.md Models.ComeGo and DTOs.ComeGo namespaces, and domain_concepts.md Section 1.5.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 PresenceRegistration, ChildStatus, ActivityListResult structs with serde Deserialize
- [x] #2 PresenceSchedule, PresenceTemplate, PresenceDay structs
- [x] #3 PickupResponsible, SleepInterval, SpareTimeActivity structs
- [x] #4 OpeningHours, ClosedDay, Location structs
- [x] #5 PresenceConfiguration, PresenceModuleSettings, PresenceFilter structs
- [x] #6 Request parameter types for presence updates
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create presence.rs with all ComeGo model structs organized by sub-namespace
2. Cover: core types (ChildStatus, PresenceRegistration), filters, activity list, schedule/templates, pickup, sleep, vacation, opening hours, configuration, employee week overview, daily overview, request types
3. Use existing enums from presence.rs, existing types from profiles.rs and messaging.rs
4. Add mod.rs entry
5. Add serde tests
6. Run e2e
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented all ComeGo model types from data_models.md.
- 60+ structs covering: core presence, activity list, filters, schedules, pickup, sleep, opening hours, closed days, configuration, daily overview, vacation, employee week overview, and request/update types.
- 14 serde deserialization tests all passing.
- References existing enums from presence.rs and types from profiles.rs/messaging.rs.
- Types not fully defined in data_models.md (ComeGoUniStudentProfile, AvailablePresenceStatusViewModel, ChildComeGoInfoViewModel, SleepIntervalsViewModel) handled with inferred shapes or serde_json::Value.
- ActivityListSortingEnum and ActivityListNoteEnum not defined in data_models.md; used String type in ActivityListRequest.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added presence.rs model module with 60+ Rust structs covering the entire ComeGo/presence domain.

Changes:
- Created aula/aula-api/src/models/presence.rs with structs organized by sub-domain: core presence types, activity list, filters, schedule/templates, pickup responsible, sleep intervals, opening hours/closed days, presence configuration, daily overview, vacation registration, employee week overview, and request/update parameter types.
- Registered module in models/mod.rs.
- All structs derive Debug, Clone, Serialize, Deserialize with camelCase serde renaming.
- References existing enum types from enums/presence.rs and model types from profiles.rs and messaging.rs.

Tests:
- 14 deserialization tests covering all major struct categories.
- Full e2e pass: 220 tests, clippy clean, fmt clean.
<!-- SECTION:FINAL_SUMMARY:END -->
