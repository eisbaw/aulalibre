---
id: TASK-0005
title: Analyze resource files and assets
status: To Do
assignee: []
created_date: '2026-03-18 13:31'
labels: []
dependencies:
  - TASK-0001
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Examine res/, assets/, and other resource directories. Identify configuration files, embedded databases, certificate pinning configs, network security configs, and any hardcoded URLs or API endpoints in resource XML/JSON.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Network security config analyzed (certificate pinning, trusted CAs)
- [ ] #2 All hardcoded URLs and API base URLs extracted from resources
- [ ] #3 Embedded databases or data files catalogued
- [ ] #4 String resources scanned for API keys, endpoints, feature flags
<!-- AC:END -->
