---
id: TASK-0046
title: Presence (ComeGo) model types
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
Rust structs for the presence/attendance domain: PresenceRegistration, ChildStatus, PresenceSchedule, PresenceTemplate, PickupResponsible, OpeningHours, ClosedDay, SleepInterval, SpareTimeActivity, Location, VacationAnnouncement, PresenceModuleSettings, DailyOverview, PresenceFilter. ComeGo is a major subsystem with 40+ API methods and its own configuration system. See data_models.md Models.ComeGo and DTOs.ComeGo namespaces, and domain_concepts.md Section 1.5.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PresenceRegistration, ChildStatus, ActivityListResult structs with serde Deserialize
- [ ] #2 PresenceSchedule, PresenceTemplate, PresenceDay structs
- [ ] #3 PickupResponsible, SleepInterval, SpareTimeActivity structs
- [ ] #4 OpeningHours, ClosedDay, Location structs
- [ ] #5 PresenceConfiguration, PresenceModuleSettings, PresenceFilter structs
- [ ] #6 Request parameter types for presence updates
<!-- AC:END -->
