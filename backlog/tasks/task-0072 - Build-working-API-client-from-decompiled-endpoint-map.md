---
id: TASK-0072
title: Build working API client from decompiled endpoint map
status: To Do
assignee: []
created_date: '2026-03-18 21:59'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The full REST API has been mapped from decompilation (see decompilation_analysis.md). 170+ endpoints using ?method= query parameter routing. Build a working API client (Rust or Python) that can authenticate and call key endpoints: getProfilesByLogin, getThreads, getAllPosts, getMedia, etc. Requires handling OAuth2 OIDC flow with SimpleSAMLphp and CSRF-P token management.
<!-- SECTION:DESCRIPTION:END -->
