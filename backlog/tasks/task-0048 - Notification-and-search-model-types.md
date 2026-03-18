---
id: TASK-0048
title: Notification and search model types
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
  - notification_messaging.md
  - domain_concepts.md
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Rust structs for notifications: NotificationItemDto (with 27+ fields including NotificationEventType with 57 values, NotificationArea, ThreadId, EventId, etc.), NotificationSettings, ComeGoNotificationSettings, WidgetNotificationSettings, ConfigureDeviceModel, SimpleDevice. Search: SearchResponse, SearchResultItem with entity type discrimination (Profile, Group, Child, Event, Post, Thread, Media, etc.), search parameters. Groups: Group, GroupMembership, SimpleGroupDto. See data_models.md and notification_messaging.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 NotificationItemDto with all fields from notification_messaging.md Section 3
- [ ] #2 NotificationSettings, device registration, and per-channel settings structs
- [ ] #3 SearchResponse, SearchResultItem with entity type variants
- [ ] #4 Group, GroupMembership, SimpleGroupDto structs
- [ ] #5 Consent, OnboardingStatus, and remaining minor model types
<!-- AC:END -->
