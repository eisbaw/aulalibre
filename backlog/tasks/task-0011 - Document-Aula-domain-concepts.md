---
id: TASK-0011
title: Document Aula domain concepts
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:32'
updated_date: '2026-03-18 15:49'
labels: []
dependencies:
  - TASK-0010
  - TASK-0008
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Map out Aula's business domain: institutions, children, parents, teachers, messages, threads, calendar events, galleries, notifications, weekly plans, etc. Understand the domain language used in the API and code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Core domain entities listed with descriptions
- [x] #2 Relationships between entities documented (parent-child, institution-user, etc.)
- [x] #3 Domain-specific terminology mapped (Danish to English where applicable)
- [x] #4 User roles and permission model documented
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Synthesize findings from api_endpoints.md, data_models.md, auth_flow.md
2. Identify core domain entities from model classes and API services
3. Map entity relationships from Model Relationships section and cross-references
4. Extract Danish terminology from enum values, class names, and UI strings
5. Document user roles from PortalRole, InstitutionRole, and PermissionEnum
6. Write domain_concepts.md with all four AC areas covered
7. Check ACs, add final summary, mark done
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Synthesized data from api_endpoints.md (22 services, 200+ operations), data_models.md (604 classes, 136 enums), and auth_flow.md
- Identified 14 core domain entity groups: Organization hierarchy, People/Profiles, Communication (Messages, Posts, Comments), Calendar/Events, Presence/ComeGo, Documents, Gallery, Notifications, Search, Widgets, Consents, Personal Reference Data, Onboarding, Files
- Mapped 40+ Danish domain terms to English equivalents with code-level references
- Documented two-dimensional role model (PortalRole x InstitutionRole) and 100+ granular permissions
- Key insight: InstitutionProfileId is the pivot entity for all API operations, not ProfileId
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created domain_concepts.md documenting Aula's business domain from reverse-engineered APK data.\n\nContents:\n- 14 core domain entities with field-level descriptions (Organization hierarchy, People/Profiles, Communication, Calendar, Presence/ComeGo, Documents, Gallery, Notifications, Search, Widgets, Consents, Personal Reference Data, Onboarding, Files)\n- Entity relationship diagrams showing organizational structure, identity model, content ownership, and presence tracking\n- 40+ Danish-to-English domain term mappings organized by category, each linked to specific enum/class references in the codebase\n- Complete role and permission model: PortalRole x InstitutionRole matrix, two-tier authentication levels (UniLogin/MitID), 100+ granular permissions grouped by domain, permission scoping model, communication blocking, and typical role capability matrix\n- 5 architectural observations about the domain model design\n\nSynthesized from api_endpoints.md, data_models.md, and auth_flow.md produced by prior tasks.
<!-- SECTION:FINAL_SUMMARY:END -->
