---
id: TASK-0002
title: Analyze AndroidManifest.xml
status: To Do
assignee: []
created_date: '2026-03-18 13:31'
labels: []
dependencies:
  - TASK-0001
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse the AndroidManifest.xml from the main APK to identify activities, services, receivers, permissions, intent filters, and content providers. This reveals the app's entry points and declared capabilities.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All activities listed with their intent filters
- [ ] #2 All services and broadcast receivers documented
- [ ] #3 Permissions catalogued (requested and declared)
- [ ] #4 Content providers and authorities identified
- [ ] #5 Main launcher activity identified
- [ ] #6 Findings documented in milestone2_analysis.md
<!-- AC:END -->
