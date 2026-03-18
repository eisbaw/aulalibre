---
id: TASK-0055
title: Presence (ComeGo) API service
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:11'
updated_date: '2026-03-18 18:53'
labels:
  - rust
  - aula-api
  - service
dependencies:
  - TASK-0049
  - TASK-0046
references:
  - api_endpoints.md
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the presence/attendance API service with 40+ methods from api_endpoints.md Section 3.6. State: GetChildrensState, GetPresenceRegistrations, GetPresenceRegistrationDetail, UpdatePresenceRegistration, UpdateStatusByPresenceRegistrationIds, UpdateStatusByInstitutionProfileIds. Schedule: GetPresenceSchedules, GetPresenceWeekOverview, UpdateOneDayPresence, GetTemplateForDate, DeleteRepeatedPresenceTemplate, GetOverlappingPresenceTemplates. Pickup: GetSuggestionsForPickUp, UpdateSuggestionsForPickup, GetPickupResponsibles, DeletePickupResponsible, GetChildGoHomeWith. Sleep: AddSleepIntervals, UpdateSleepInterval, DeleteSleepIntervals. Activity: GetActivityList, GetActivityFilter, GetDailyOverview. Location: GetAvailableLocations, UpdateLocation. Vacation: AddVacation, GetChildrenVacation, GetVacationAnnouncementsByChildren, GetVacationRegistrationOverview, GetVacationRegistrationsByChildren. Config: GetPresenceConfiguration, GetPresenceConfigurationByChildrenIds, GetPresenceFilter(s), GetClosedDays, GetGeneralOpeningHours, GetOpeningHoursByInstitutionCodes, GetSpecificOpeningHourOverview, GetAvailablePresenceStatuses, GetInstitutionWithPresenceStates, GetPresenceChildrenDistribution.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Child state and registration: get state, list/detail/update registrations, bulk status updates
- [x] #2 Schedule and templates: week overview, one-day update, template CRUD
- [x] #3 Pickup management: suggestions, responsibles, child go-home-with
- [x] #4 Sleep interval CRUD and activity/daily overview
- [x] #5 Location and vacation management
- [x] #6 Configuration: presence config, filters, opening hours, closed days, statuses
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create presence.rs service file following calendar.rs pattern
2. Implement AC#1: Child state/registration methods (GetChildrensState, GetPresenceRegistrations, GetPresenceRegistrationDetail, UpdatePresenceRegistration, UpdateStatusByPresenceRegistrationIds, UpdateStatusByInstitutionProfileIds)
3. Implement AC#2: Schedule/template methods (GetPresenceSchedules, GetPresenceWeekOverview, UpdateOneDayPresence, GetTemplateForDate, DeleteRepeatedPresenceTemplate, GetOverlappingPresenceTemplates)
4. Implement AC#3: Pickup methods (GetSuggestionsForPickUp, UpdateSuggestionsForPickup, GetPickupResponsibles, DeletePickupResponsible, GetChildGoHomeWith)
5. Implement AC#4: Sleep/activity methods (AddSleepIntervals, UpdateSleepInterval, DeleteSleepIntervals, GetActivityList, GetActivityFilter, GetDailyOverview)
6. Implement AC#5: Location/vacation methods (GetAvailableLocations, UpdateLocation, AddVacation, GetChildrenVacation, GetVacationAnnouncementsByChildren, GetVacationRegistrationOverview, GetVacationRegistrationsByChildren)
7. Implement AC#6: Configuration methods (GetPresenceConfiguration, GetPresenceConfigurationByChildrenIds, GetPresenceFilter(s), GetClosedDays, GetGeneralOpeningHours, GetOpeningHoursByInstitutionCodes, GetSpecificOpeningHourOverview, GetAvailablePresenceStatuses, GetInstitutionWithPresenceStates, GetPresenceChildrenDistribution)
8. Add unit tests for serialization
9. Register module in mod.rs
10. Run just e2e
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented all 40+ presence service methods in aula-api/src/services/presence.rs.
All methods follow the established patterns from calendar.rs and messaging.rs.
16 unit tests for request serialization added and passing.
Full e2e suite (388 tests) passes with clippy and fmt clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented the Presence (ComeGo) API service covering all 40+ methods from PresenceWebService.

Changes:
- Created aula-api/src/services/presence.rs with 40+ async methods organized into 6 sections:
  1. Child state and registration (get_childrens_state, get/update presence registrations, bulk status updates)
  2. Schedule and templates (schedules, week overview, one-day presence, template CRUD)
  3. Pickup management (suggestions, responsibles, child go-home-with)
  4. Sleep intervals and activity (add/update/delete sleep, activity list/filter, daily overview)
  5. Location and vacation (locations, vacation entries, announcements, registration overview)
  6. Configuration (presence config, filters, opening hours, closed days, statuses, distribution)
- Registered presence module in services/mod.rs
- All methods use typed request/response models from models/presence.rs (60+ structs)
- Query parameter construction follows established patterns from calendar.rs

Tests:
- 16 unit tests for request type serialization
- Full e2e suite: 388 tests pass, clippy clean, rustfmt clean
<!-- SECTION:FINAL_SUMMARY:END -->
