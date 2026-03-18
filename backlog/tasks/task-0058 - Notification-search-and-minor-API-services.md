---
id: TASK-0058
title: 'Notification, search, and minor API services'
status: To Do
assignee: []
created_date: '2026-03-18 16:12'
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
- [ ] #1 Notification service: list, delete, delete for child
- [ ] #2 Push notification service: device registration, settings, badge clearing
- [ ] #3 Search service: global search, per-entity search, recipient search
- [ ] #4 Group service: get, get by context, memberships, join/leave
- [ ] #5 Minor services: isAlive, onboarding, consent, personal reference, widget token
<!-- AC:END -->
