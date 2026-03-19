---
id: TASK-0035
title: Investigate ESDH integration and journaling flow
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 15:50'
updated_date: '2026-03-19 05:55'
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

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed the ESDH (Elektronisk Sags- og Dokumenthandtering) integration in Aula's secure documents module.

Key findings:
- The app is a thin client for ESDH -- no direct submission logic, only status tracking
- JournalingStatusEnum tracks four states: NotProcessed, InProgress, Failed, Completed
- Six revision change types capture the full ESDH lifecycle including retry and manual fallback
- JOURNALING_TO_ESDH permission (ID 132) gates per-institution access to journaling filters
- During journaling, recipient names and children names are sent to ESDH (visible in revision history)
- UI presents journaling as "publishing" with three filter options mapping to journaling statuses
- Document categories map to formal Danish municipal document types (Handleplan, Magtanvendelsesskema, etc.)
- Export (PDF download) and journaling (ESDH submission) are independent workflows with separate permissions

Output: esdh_journaling_analysis.md with complete state machine, permission model, API endpoints, data exposure analysis, and source file references.
<!-- SECTION:FINAL_SUMMARY:END -->
