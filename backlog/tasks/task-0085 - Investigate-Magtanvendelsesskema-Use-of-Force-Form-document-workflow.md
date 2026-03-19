---
id: TASK-0085
title: Investigate Magtanvendelsesskema (Use of Force Form) document workflow
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 05:55'
updated_date: '2026-03-19 07:33'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The DocumentCategoryEnum includes 'Magtanvendelsesskema' (Use of Force Form) as a secure document category. This is a legally significant document type in Danish child services (Serviceloven). Worth investigating: what triggers creation, who has access, is it auto-journalized to ESDH, what data does it contain, and how does it flow through the system. Discovered during TASK-35 ESDH analysis.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Identify all code references to Magtanvendelse/UseOfForce in decompiled sources
- [x] #2 Map the document creation flow (triggers, form fields, data model)
- [x] #3 Map access control (who can create, view, approve these forms)
- [x] #4 Determine ESDH integration (auto-journalization, archive flow)
- [x] #5 Document API endpoints involved in the workflow
- [x] #6 Document findings in a structured analysis
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Search decompiled sources for Magtanvendelse/ForCableSchedule references
2. Map document data model (SecureDocumentDto, CreateDocumentArguments)
3. Map document category system and how Magtanvendelsesskema fits
4. Map access control (PermissionManager, StepUp auth, OnlyStaffCategories)
5. Map API endpoints involved (documents.* RPC calls)
6. Map ESDH journaling integration
7. Document findings in magtanvendelse_analysis.md
8. Git commit
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed Magtanvendelsesskema (Use of Force Form) document workflow in the Aula app.

Key findings:
- Magtanvendelsesskema is a category label on generic secure documents, not a specialized form
- The enum name ForCableSchedule is a decompiler mistranslation artifact
- Staff-only category (employees only can create/select it)
- Requires HANDLE_SECURE_FILES permission plus non-child or stepped-up auth
- No structured data capture in mobile client -- form is free-text rich content
- Server-driven ESDH journalization (no client-side submission logic)
- Implicit sharing system auto-calculates access for related guardians/staff
- All operations use the same documents.* RPC API as other secure document categories
- No dedicated API endpoints for this document type

Produced: magtanvendelse_analysis.md with full workflow documentation
<!-- SECTION:FINAL_SUMMARY:END -->
