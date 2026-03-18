---
id: TASK-0055
title: Presence (ComeGo) API service
status: To Do
assignee: []
created_date: '2026-03-18 16:11'
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
- [ ] #1 Child state and registration: get state, list/detail/update registrations, bulk status updates
- [ ] #2 Schedule and templates: week overview, one-day update, template CRUD
- [ ] #3 Pickup management: suggestions, responsibles, child go-home-with
- [ ] #4 Sleep interval CRUD and activity/daily overview
- [ ] #5 Location and vacation management
- [ ] #6 Configuration: presence config, filters, opening hours, closed days, statuses
<!-- AC:END -->
