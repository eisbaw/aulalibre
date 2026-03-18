---
id: TASK-0012
title: Identify push notification and real-time messaging
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:32'
updated_date: '2026-03-18 16:04'
labels: []
dependencies:
  - TASK-0006
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Analyze how the app receives push notifications and real-time updates. Look for Firebase Cloud Messaging, WebSocket connections, Server-Sent Events, or polling mechanisms.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Push notification provider identified (FCM, etc.)
- [x] #2 Real-time update mechanism documented (WebSocket, SSE, polling)
- [x] #3 Notification payload format documented
- [x] #4 Subscription/channel model for notifications documented
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Analyze AulaFirebaseMessagingService and Firebase config (AC#1)
2. Search for WebSocket, SignalR, SSE, polling - identify real-time mechanism (AC#2)
3. Document notification payload format from model classes (AC#3)
4. Document subscription/channel model from enums and settings (AC#4)
5. Write notification_messaging.md analysis document
6. Create tangent tasks for follow-up items
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Analysis complete. Key findings:
- Push notifications: FCM via aula-private Firebase project
- Real-time: NO WebSocket/SignalR/SSE -- pure FCM push + REST polling
- Payload: FCM data messages with notificationType, notificationArea, notificationId, institutionCode, and entity-specific IDs
- Subscription: Per-user NotificationSettings with content channels, day-of-week filters, instant vs scheduled delivery, and per-widget/ComeGo settings
- 50 RemoteNotificationType values, 12 NotificationArea modules, 57 in-app NotificationEventType values
- Internal MessagingCenter pub/sub bus (20+ channels) for cross-component event distribution
- monodis segfaults prevented full IL decompilation of assembly_0 (Droid), but typedef/fields extraction was sufficient
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed push notification and real-time messaging architecture of the Aula Android app.

Key findings:
- Push provider: Firebase Cloud Messaging (FCM) via project "aula-private" (sender ID 811573413698)
- Real-time mechanism: FCM push + REST API polling. No WebSocket, SignalR, or SSE found despite WebSocket client assembly being present as framework dependency
- Notification payload: FCM data messages containing notificationType, notificationArea, notificationId, institutionCode, and entity-specific IDs (threadId, albumId, mediaId, widgetId)
- 50 RemoteNotificationType values covering messages, calendar events, posts, gallery, documents, presence, schedule, widgets, and file scan failures
- Subscription model: Per-user NotificationSettings with content channel toggles (messages, calendar, gallery, posts, etc.), day-of-week filters, instant vs scheduled delivery, and per-widget/ComeGo granular settings
- Device registration via POST /pushNotifications/devices with FCM token, device ID, platform
- Internal MessagingCenter pub/sub bus with 20+ channels for cross-component event distribution

Produced: notification_messaging.md
Created follow-up tasks: TASK-37 (Firebase RTDB investigation), TASK-38 (IL decompilation of parser), TASK-39 (polling interval analysis)
<!-- SECTION:FINAL_SUMMARY:END -->
