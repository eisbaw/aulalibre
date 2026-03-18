---
id: TASK-0042
title: Core domain enums
status: To Do
assignee: []
created_date: '2026-03-18 16:08'
labels:
  - rust
  - aula-api
  - models
dependencies:
  - TASK-0040
references:
  - data_models.md
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Translate the 136 enum types from data_models.md into Rust enums with serde Serialize/Deserialize. Group by domain: calendar (EventClass, EventType, LessonStatus, ResponseType, RepeatType, TimeslotResponseType, ParticipantRole), presence/ComeGo (PresenceStatusEnum, ActivityTypeEnum, OpeningHoursType, PresenceDayOfWeek, PresenceModuleSettingsModule/Permission/Dashboard), messaging (MessageType, SensitivityLevel, SubscriptionStatus, RecipientType), profiles (PortalRole, InstitutionRole, InstitutionTypeEnum, GroupRole, GroupStatus, UserRelationType), documents (DocumentTypeEnum, DocumentCategoryEnum), gallery (MediaTypeEnum, ConversionStatus), notifications (NotificationArea, NotificationType, NotificationEventType, RemoteNotificationType), and general (Platform, WeekDay, SortOrderEnum, etc.).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All 136 enums from data_models.md translated to Rust with serde derive
- [ ] #2 Enums organized into domain modules: calendar, presence, messaging, profiles, documents, gallery, notifications, common
- [ ] #3 Enums use #[serde(rename_all)] where API naming convention differs from Rust convention
- [ ] #4 Unit tests verify round-trip serialization for each enum
<!-- AC:END -->
