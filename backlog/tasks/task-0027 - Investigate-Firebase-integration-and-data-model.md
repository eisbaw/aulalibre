---
id: TASK-0027
title: Investigate Firebase integration and data model
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-18 14:35'
updated_date: '2026-03-19 05:41'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The app uses Firebase project 'aula-private' with Realtime Database (aula-private.firebaseio.com), Cloud Messaging, and Storage (aula-private.appspot.com). Investigate what data flows through Firebase vs the main API, whether push notification payloads contain sensitive data, and the Firebase security rules.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Firebase data model documented (what data goes through Firebase)
- [ ] #2 FCM push notification payload structure analyzed
- [ ] #3 Firebase security rules probed (public read access check)
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
