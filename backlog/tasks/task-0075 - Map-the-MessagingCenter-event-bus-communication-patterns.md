---
id: TASK-0075
title: Map the MessagingCenter event bus communication patterns
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 23:31'
updated_date: '2026-03-19 06:16'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
25 domain-specific MessagingCenter classes form an event bus. Trace which components publish and subscribe to understand implicit coupling. Key centers: LoginMessagingCenter, PostMessagingCenter, NotificationMessagingCenter, etc.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Catalog all 23 domain-specific MessagingCenter wrapper classes with their message keys and payload types
- [x] #2 Map every publisher (Notify/Send call site) to its source file and component
- [x] #3 Map every subscriber (Subscribe call site) to its source file and component
- [x] #4 Categorize messages by domain (auth, messaging, gallery, documents, notifications, UI/navigation, ComeGo/attendance)
- [x] #5 Identify dead/unused channels (defined but no subscribers or no publishers in Android)
- [x] #6 Document findings in a structured analysis file
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read all MessagingCenter wrapper class definitions (done)
2. Grep all Notify/Send call sites to find publishers (done)
3. Grep all Subscribe/Unsubscribe call sites to find subscribers (done)
4. Build structured mapping: message key -> payload -> publishers -> subscribers
5. Categorize by domain
6. Identify dead channels
7. Write analysis document
8. Commit
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Mapped the complete MessagingCenter event bus architecture from decompiled C# source.

Changes:
- Created messagingcenter_event_bus.md documenting all 23 wrapper classes, 30 message keys, publishers, subscribers, and payload types
- Organized by domain: auth/session, messaging/threads, posts, notifications, gallery/media, documents, UI/navigation, institution, ComeGo/attendance, delegate access
- Identified 7 dead/unused channels (defined but missing publishers or subscribers in Android)

Key findings:
- FilterProfileBarMessagingCenter has the highest fan-out (18 subscribers) -- the profile/child switch mechanism
- SubscriptionMessagingCenter.ThreadChange has the highest fan-in (16+ publishers) -- most complex message flow
- Dominant pattern: ServiceManagers publish after API calls, Fragments/ViewModels subscribe for UI refresh
- LoginFinishedMessagingCenter uses async TaskCompletionSource gate pattern for push notification handling
<!-- SECTION:FINAL_SUMMARY:END -->
