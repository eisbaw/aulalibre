---
id: TASK-0072
title: Build working API client from decompiled endpoint map
status: Done
assignee: []
created_date: '2026-03-18 21:59'
updated_date: '2026-03-19 18:53'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The full REST API has been mapped from decompilation (see decompilation_analysis.md). 170+ endpoints using ?method= query parameter routing. Build a working API client (Rust or Python) that can authenticate and call key endpoints: getProfilesByLogin, getThreads, getAllPosts, getMedia, etc. Requires handling OAuth2 OIDC flow with SimpleSAMLphp and CSRF-P token management.
<!-- SECTION:DESCRIPTION:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed across multiple tasks (TASK-0092 through TASK-0099). The Rust API client in aula-api/ can authenticate, initialize sessions, and call key endpoints: getProfilesByLogin, getProfileContext, getThreads, getCalendarEvents, getPresenceStatus, search, gallery, posts, notifications, and more. OAuth2 OIDC flow with cookie-based session management is implemented in aula-cli.
<!-- SECTION:FINAL_SUMMARY:END -->
