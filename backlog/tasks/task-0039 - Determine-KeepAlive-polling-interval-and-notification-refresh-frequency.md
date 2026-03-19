---
id: TASK-0039
title: Determine KeepAlive polling interval and notification refresh frequency
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:04'
updated_date: '2026-03-19 06:05'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The ProfileServiceManager.KeepAlive() and notification refresh methods have intervals set in method bodies, not metadata. Decompile these methods to determine how frequently the app polls the server. Important for understanding server load patterns and battery impact.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Identify KeepAlive polling interval in seconds/minutes
- [x] #2 Identify notification refresh polling interval
- [x] #3 Document the session keepalive mechanism (what endpoint is called, what triggers it)
- [x] #4 Document the notification refresh flow (endpoint, trigger, scheduling)
- [x] #5 Identify any background polling (WorkManager, AlarmManager, timers)
- [x] #6 Document findings in architecture or analysis docs
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Search decompiled C# code for KeepAlive, polling timers, notification refresh patterns
2. Trace the KeepAlive call chain (ProfileService -> ProfileServiceManager -> callers)
3. Trace notification refresh/badge update flow
4. Identify all AutoRefreshManager and AulaTaskScheduler intervals
5. Document session expiration and keepalive mechanism
6. Document findings in analysis doc
7. Commit
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed KeepAlive mechanism, polling intervals, and notification refresh patterns from decompiled C# source.

Key findings:
- KeepAlive (`session.keepAlive`) is on-demand only, not timer-polled. Called reactively before device unregistration when no CSRF token exists.
- No background polling (no WorkManager, AlarmManager, or background timers).
- Notifications delivered via Firebase Cloud Messaging (FCM push). Notification cache lifetime is 0.3 seconds (effectively no cache).
- Session timeout is 60 minutes idle, with warning dialog at 55 minutes. Client-side 1-second timer tracks idle time.
- Gallery is the only auto-refreshing UI (15 min via AutoRefreshManager).
- BaseDataManager throttles repeated API calls at 1-minute minimum intervals.

Added: keepalive_polling_analysis.md
<!-- SECTION:FINAL_SUMMARY:END -->
