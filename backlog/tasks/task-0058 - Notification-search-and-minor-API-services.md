---
id: TASK-0058
title: 'Notification, search, and minor API services'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:12'
updated_date: '2026-03-18 19:15'
labels:
  - rust
  - aula-api
  - service
dependencies:
  - TASK-0049
  - TASK-0048
references:
  - api_endpoints.md
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement remaining API services. Notifications (3 methods, Section 3.16): GetNotificationsForActiveProfile, DeleteNotifications, DeleteNotificationForRelatedChild. Push notifications (7 methods, Section 3.17): RegisterDevice, UnregisterDevice, DeleteAllDevices, GetDevices, GetNotificationSettings, UpdateNotificationSettings, ClearNotificationBadgesByModule. Search (9 methods, Section 3.15): GlobalSearch, SearchForMessages, SearchForProfiles, SearchForProfilesAndGroups, SearchForRecipients (3 variants), SearchForGroupsToAssociateDocument, SearchGroups. Groups (4 methods, Section 3.14): GetGroup, GetGroupByContext, GetMembershipsLight, JoinOrLeaveGroup. Minor services: IsAlive (3.1), Onboarding (3.18), Consent (3.19), PersonalReference (3.20), Widget token (3.21).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Notification service: list, delete, delete for child
- [x] #2 Push notification service: device registration, settings, badge clearing
- [x] #3 Search service: global search, per-entity search, recipient search
- [x] #4 Group service: get, get by context, memberships, join/leave
- [x] #5 Minor services: isAlive, onboarding, consent, personal reference, widget token
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create notifications.rs service (3 methods: list, delete, delete for child)
2. Create push_notifications.rs service (7 methods: register/unregister/delete devices, get devices, get/update settings, clear badges)
3. Create search.rs service (9 methods: global, messages, profiles, profilesAndGroups, recipients x3, groups x2)
4. Create groups.rs service (4 methods: get, getByContext, memberships, join/leave)
5. Create minor services: health.rs (isAlive), onboarding.rs, consent.rs, personal_reference.rs, widget.rs
6. Register all new service modules in services/mod.rs
7. Run just e2e, fix issues
8. Mark ACs done
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented all 9 service modules:
- notifications.rs (3 methods)
- push_notifications.rs (7 methods + ClearBadgesRequest type)
- search.rs (9 methods with query-string building)
- groups.rs (4 methods + JoinOrLeaveGroupRequest type)
- health.rs (isAlive + IsAliveResponse type)
- onboarding.rs (2 methods + PolicyLink type)
- consent.rs (2 methods)
- personal_reference.rs (3 methods, uses serde_json::Value for unknown response types)
- widget.rs (1 method + WidgetTokenResponse type)
All registered in services/mod.rs. 439 tests passing, clippy clean, fmt clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented all remaining API service modules for the aula-api crate, completing the full Aula REST API surface.

New service modules (9 files):
- notifications.rs: GET/DELETE /notifications, DELETE /notifications/child/{id}
- push_notifications.rs: CRUD for devices, get/update settings, clear badges (7 endpoints)
- search.rs: global search, per-entity search (messages, profiles, groups), recipient search variants (9 endpoints)
- groups.rs: get group, get by context, memberships, join/leave (4 endpoints)
- health.rs: isAlive health check
- onboarding.rs: mark complete, get policy links
- consent.rs: get/post consents
- personal_reference.rs: additional answers, consent answers, questions (uses Value for unknown response schemas)
- widget.rs: get widget SSO token

All modules follow the established service patterns (query-string building for GETs, typed request/response bodies). Models from existing model files (notifications.rs, search.rs, consent.rs, onboarding.rs, groups.rs) are reused; new request types added where needed (ClearBadgesRequest, JoinOrLeaveGroupRequest, PolicyLink, WidgetTokenResponse, IsAliveResponse).

Tests: 439 passing (was 417 before this task). Clippy + fmt clean.
<!-- SECTION:FINAL_SUMMARY:END -->
