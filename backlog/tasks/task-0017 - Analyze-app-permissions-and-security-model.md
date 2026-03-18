---
id: TASK-0017
title: Analyze app permissions and security model
status: To Do
assignee: []
created_date: '2026-03-18 13:59'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Aula app requests notable permissions: CAMERA, CALL_PHONE, USE_BIOMETRIC/USE_FINGERPRINT, SYSTEM_ALERT_WINDOW, POST_NOTIFICATIONS, plus Firebase C2DM for push. Also has a Samsung-specific analytics permission. Investigate what each permission is used for and whether any are over-privileged.
<!-- SECTION:DESCRIPTION:END -->
