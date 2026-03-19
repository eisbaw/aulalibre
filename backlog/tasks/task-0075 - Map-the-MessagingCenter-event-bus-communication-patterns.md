---
id: TASK-0075
title: Map the MessagingCenter event bus communication patterns
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-18 23:31'
updated_date: '2026-03-19 06:13'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
25 domain-specific MessagingCenter classes form an event bus. Trace which components publish and subscribe to understand implicit coupling. Key centers: LoginMessagingCenter, PostMessagingCenter, NotificationMessagingCenter, etc.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Catalog all 23 domain-specific MessagingCenter wrapper classes with their message keys and payload types
- [ ] #2 Map every publisher (Notify/Send call site) to its source file and component
- [ ] #3 Map every subscriber (Subscribe call site) to its source file and component
- [ ] #4 Categorize messages by domain (auth, messaging, gallery, documents, notifications, UI/navigation, ComeGo/attendance)
- [ ] #5 Identify dead/unused channels (defined but no subscribers or no publishers in Android)
- [ ] #6 Document findings in a structured analysis file
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
