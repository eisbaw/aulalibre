---
id: TASK-0030
title: Network traffic capture of Aula API
status: To Do
assignee: []
created_date: '2026-03-18 15:08'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Use mitmproxy or similar to capture actual API requests from the running Aula app. This will validate the endpoint paths inferred from decompilation and capture actual JSON request/response bodies, headers, and authentication tokens in action. Requires: rooted Android device or emulator, certificate installation for HTTPS interception.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 mitmproxy or equivalent proxy tool set up
- [ ] #2 At least 20 distinct API endpoints captured with full request/response
- [ ] #3 Captured traffic documented and cross-referenced with api_endpoints.md
<!-- AC:END -->
