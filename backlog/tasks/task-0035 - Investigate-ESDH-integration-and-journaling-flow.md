---
id: TASK-0035
title: Investigate ESDH integration and journaling flow
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-18 15:50'
updated_date: '2026-03-19 05:54'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The secure documents module has deep integration with ESDH (Elektronisk Sags- og Dokumenthandtering) municipal records systems. DocumentCategoryEnum and JournalingStatusEnum suggest a complex workflow for exporting child-related documents to official municipal records. Worth tracing the full journaling flow and what data leaves Aula.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Map the complete ESDH journaling state machine (statuses, transitions, revision events)
- [x] #2 Document the permission model gating ESDH functionality (JOURNALING_TO_ESDH permission)
- [x] #3 Document the API endpoints involved in document export/journaling flow
- [x] #4 Document the data model: what data leaves Aula during journaling (recipient, children names)
- [x] #5 Document the UI filter mapping (Published/PublishFailed/PublishInProgress to JournalingStatus)
- [x] #6 Write findings to a document in the repo
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Trace all ESDH-related code paths from decompiled C# sources
2. Map the journaling state machine from enums and revision history
3. Identify permission gating and UI filtering logic
4. Document what data leaves Aula (recipient names, children names)
5. Write comprehensive analysis document
6. Check for follow-up tangents to capture as new tasks
<!-- SECTION:PLAN:END -->
