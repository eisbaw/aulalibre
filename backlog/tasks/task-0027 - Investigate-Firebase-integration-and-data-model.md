---
id: TASK-0027
title: Investigate Firebase integration and data model
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 14:35'
updated_date: '2026-03-19 05:44'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The app uses Firebase project 'aula-private' with Realtime Database (aula-private.firebaseio.com), Cloud Messaging, and Storage (aula-private.appspot.com). Investigate what data flows through Firebase vs the main API, whether push notification payloads contain sensitive data, and the Firebase security rules.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Firebase data model documented (what data goes through Firebase)
- [x] #2 FCM push notification payload structure analyzed
- [x] #3 Firebase security rules probed (public read access check)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Search decompiled source for all Firebase-related code
2. Analyze FirebaseNotificationParser and notification handling flow
3. Check if Firebase RTDB or Storage is actually used in code (vs just configured)
4. Probe Firebase endpoints for public access
5. Document findings in firebase_analysis.md
6. Commit and mark ACs complete
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed Firebase integration in the Aula Android app (com.netcompany.aulanativeprivate v2.15.4).

Key findings:
- Firebase is used ONLY for FCM push notification delivery. Realtime Database and Storage are configured in the Firebase project (aula-private) but have zero code-level usage in the decompiled .NET assemblies.
- FCM payloads are privacy-conscious: the data payload contains only opaque IDs (elementId, relatedChildInstitutionProfileId, commonInboxId, etc.) and a type string. No message content, names, or PII. The notification body text (for system tray) may contain preview text.
- 47 distinct notification types identified covering messages, calendar events, meetings, posts, gallery, presence, and file scan failures.
- All Firebase endpoints properly secured: RTDB returns 401 (Permission denied) on all probed paths; Storage returns 412 (service account issue, no data exposure).
- Full notification flow documented: server sends FCM with IDs -> app receives and parses -> triggers UI refresh -> app fetches actual content from Aula API.
- Device registration sends FCM token to Aula backend via notifications.registerDevice API, standard server-side FCM pattern.

Created: firebase_analysis.md with complete data model, payload structure, security probe results, and data flow diagram.
<!-- SECTION:FINAL_SUMMARY:END -->
