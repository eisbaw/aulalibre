---
id: TASK-0048
title: Notification and search model types
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:09'
updated_date: '2026-03-18 18:05'
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
- [x] #1 NotificationItemDto with all fields from notification_messaging.md Section 3
- [x] #2 NotificationSettings, device registration, and per-channel settings structs
- [x] #3 SearchResponse, SearchResultItem with entity type variants
- [x] #4 Group, GroupMembership, SimpleGroupDto structs
- [x] #5 Consent, OnboardingStatus, and remaining minor model types
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create notifications.rs with NotificationItemDto (all 50+ fields), NotificationSettings, ComeGoNotificationSettings, WidgetNotificationSettings, ConfigureDeviceModel, SimpleDevice, RemoteNotification
2. Create search.rs with SearchResponse, SearchResultItem base, all SearchResult* subtypes, search parameter models, message search types
3. Create consent.rs with ConsentResponsesDTO, InstitutionProfileConsentDTO, ConsentUpdateDTO, ProfileConsentUpdatesDTO
4. Create onboarding.rs with OnboardingProfileDto, OnboardingResponseDto, StubbedChild, StubbedInstitutionProfile
5. Register all new modules in mod.rs
6. Run just e2e to verify compilation and tests pass
7. Mark ACs complete
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Created 4 new model files:
- notifications.rs: NotificationItemDto (50+ fields), NotificationSettings, ComeGoNotificationSettings, WidgetNotificationSettings, ConfigureDeviceModel, SimpleDevice, RemoteNotification
- search.rs: SearchResponse, SearchResultItem base + 10 specialized result types, 6 search parameter models, message search types
- consent.rs: ConsentResponsesDto, InstitutionProfileConsentDto, ConsentUpdateDto, ProfileConsentUpdatesDto
- onboarding.rs: OnboardingResponseDto, OnboardingProfileDto, StubbedChild, StubbedInstitutionProfile

All 260 tests pass (was 240 before). Clippy clean. Format clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added Rust model types for notification, search, consent, and onboarding domains.

Changes:
- notifications.rs: NotificationItemDto with all 50+ fields from data_models.md, NotificationSettings with day-of-week/content-channel/platform toggles and nested ComeGoNotificationSettings/WidgetNotificationSettings/SimpleDevice, ConfigureDeviceModel for FCM device registration, RemoteNotification for parsed push payloads
- search.rs: SearchResponse + SearchResultItem base type + 10 specialized result types (CommonFile, CommonInbox, Event, Group, Media, Post, ProfileItemBase, SecureFile, message variants), 6 search parameter models (GlobalSearchParameters, SearchRecipientParameters, SearchMessageRequestModel, etc.), group search result models
- consent.rs: ConsentResponsesDto, InstitutionProfileConsentDto with profile reference, ConsentUpdateDto and ProfileConsentUpdatesDto for batch consent updates
- onboarding.rs: OnboardingResponseDto, OnboardingProfileDto with data policy/consent age/children/institution profiles, StubbedChild and StubbedInstitutionProfile
- mod.rs: registered all 4 new modules

Tests: 260 pass (20 new), clippy clean, format clean.
<!-- SECTION:FINAL_SUMMARY:END -->
