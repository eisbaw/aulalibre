---
id: TASK-0086
title: Investigate document locking and journaling interaction
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 05:55'
updated_date: '2026-03-19 07:38'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Documents can be locked (IsLocked/CanEditLockedStatus) and journalized to ESDH. The revision history tracks both Locked and JournalizedToESDH events sequentially, suggesting auto-locking after journalization. Worth investigating: is locking automatic upon ESDH submission, can locked documents be re-journalized, what happens if a locked document's ESDH submission fails. Discovered during TASK-35 ESDH analysis.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Map the document locking data model (IsLocked, CanEditLockedStatus, UpdateDocumentStatusRequestModel)
- [x] #2 Trace the lock/unlock toggle flow from UI through service layer to API
- [x] #3 Document how locking blocks sharing (ValidateSharingSecureDocuments)
- [x] #4 Map interaction between locking and ESDH journaling filters
- [x] #5 Identify what revision history events are recorded for lock/unlock/ESDH
- [x] #6 Document the permission model for lock status changes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Search decompiled code for all locking-related types and properties
2. Trace the lock toggle flow: UI -> ViewModel -> ServiceManager -> Service -> API
3. Analyze how IsLocked interacts with sharing validation
4. Map the filter system showing locked+journaling as parallel filters
5. Document revision history events for both locking and ESDH
6. Analyze permission model (CanEditLockedStatus, IsEmployee)
7. Write analysis document
8. Commit
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Investigated document locking mechanisms in Aula's secure documents module and their interaction with ESDH journaling.

Key findings:
- Locking and ESDH journaling are independent mechanisms sharing the same SecureDocumentDto carrier
- No auto-locking on journalization exists in app code (server may enforce this)
- Locking prevents sharing (client-side validation) but does not client-side block editing, deletion, or PDF export
- AllowToEdit does NOT check IsLocked; the server communicates edit restrictions via the CanEdit flag
- Lock toggle requires both server-granted CanEditLockedStatus and employee role
- Filters are mutually exclusive: cannot filter "Locked AND Published" simultaneously
- Both lock and ESDH events are tracked in the same revision history system

Added: document_locking_analysis.md with full trace through data model, UI flow, service layer, API endpoint, permission model, and filter system.
<!-- SECTION:FINAL_SUMMARY:END -->
