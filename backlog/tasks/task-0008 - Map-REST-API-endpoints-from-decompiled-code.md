---
id: TASK-0008
title: Map REST API endpoints from decompiled code
status: To Do
assignee: []
created_date: '2026-03-18 13:31'
labels: []
dependencies:
  - TASK-0006
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Search decompiled source for HTTP client usage, Retrofit/OkHttp annotations, URL patterns, and API endpoint definitions. Build a complete map of all REST API endpoints the app communicates with.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All API base URLs identified (production, staging, etc.)
- [ ] #2 Every REST endpoint documented with HTTP method, path, and parameters
- [ ] #3 Request/response model classes identified for each endpoint
- [ ] #4 Authentication headers and token handling documented
- [ ] #5 API versioning scheme identified
- [ ] #6 Findings documented in a structured format (e.g. api_endpoints.md)
<!-- AC:END -->
