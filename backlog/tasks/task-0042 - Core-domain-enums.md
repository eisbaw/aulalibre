---
id: TASK-0042
title: Core domain enums
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:08'
updated_date: '2026-03-18 17:19'
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
- [x] #1 All 136 enums from data_models.md translated to Rust with serde derive
- [x] #2 Enums organized into domain modules: calendar, presence, messaging, profiles, documents, gallery, notifications, common
- [x] #3 Enums use #[serde(rename_all)] where API naming convention differs from Rust convention
- [x] #4 Unit tests verify round-trip serialization for each enum
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add serde_repr dependency to aula-api/Cargo.toml
2. Create src/enums/ module directory with mod.rs
3. Create domain sub-modules: calendar.rs, presence.rs, messaging.rs, profiles.rs, documents.rs, gallery.rs, notifications.rs, common.rs
4. Translate all 136 enums from data_models.md into Rust enums with appropriate derives
5. Use serde rename attributes for SCREAMING_SNAKE_CASE and other non-PascalCase variants
6. Add round-trip serialization tests in each module
7. Wire enums module into lib.rs
8. Run just e2e to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All 136 enums from data_models.md translated to Rust.
Organized into 8 domain modules: calendar (15), common (36), documents (17), gallery (9), messaging (17), notifications (6), presence (19), profiles (17).
Used serde rename attributes for SCREAMING_SNAKE_CASE (ActivityTypeEnum, ComeGoNotificationEnum, SendMessageButton, CacheType, Consent, PermissionEnum), camelCase (ComeGoEmployeeWeekOverviewTenseEnum), and individual renames (GroupMembershipRole::ApplicationRemoved, GroupTypeEnum::CrossInstitutional, Platform::Ios, UpdateProfileInformationReturnCodeEnum::WrongUserTypeLoggedInAsOtp).
160 unit tests pass including roundtrip serialization and rename verification.
just e2e passes: build, test, clippy, fmt-check all green.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Translated all 136 .NET enum types from data_models.md into idiomatic Rust enums with serde Serialize/Deserialize.

Changes:
- Added serde_repr dependency to aula-api/Cargo.toml
- Created src/enums/ module with 8 domain sub-modules:
  - calendar.rs (15 enums): EventClass, EventType, LessonStatus, ParticipantRole, RepeatType, ResponseType, etc.
  - presence.rs (19 enums): PresenceStatusEnum, ActivityTypeEnum, OpeningHoursType, ComeGo tabs/notifications, etc.
  - messaging.rs (17 enums): MessageType, SensitivityLevel, SubscriptionStatus, RecipientType, ThreadType, etc.
  - profiles.rs (17 enums): PortalRole, InstitutionRole, GroupRole, GroupStatus, OnboardingStep, etc.
  - documents.rs (17 enums): DocumentTypeEnum, DocumentCategoryEnum, FileScanningStatus, CloudStorageFileType, etc.
  - gallery.rs (9 enums): MediaTypeEnum, ConversionStatusEnum, ThumbnailSizeEnum, RotatingEnum, etc.
  - notifications.rs (6 enums): NotificationArea, NotificationEventType (58 variants), RemoteNotificationType (48 variants), etc.
  - common.rs (36 enums): Platform, WeekDay, SortOrderEnum, PermissionEnum (97 variants), SearchResultItemType, etc.
- All enums derive Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize
- Applied serde rename attributes where API naming differs from Rust convention
- Wired enums module into lib.rs

Tests:
- 160 unit tests: roundtrip JSON serialization for every enum plus targeted rename verification
- All pass via just e2e (build + test + clippy + fmt-check)
<!-- SECTION:FINAL_SUMMARY:END -->
