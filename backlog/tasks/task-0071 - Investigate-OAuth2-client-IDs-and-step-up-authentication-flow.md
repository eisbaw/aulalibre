---
id: TASK-0071
title: Investigate OAuth2 client IDs and step-up authentication flow
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 21:59'
updated_date: '2026-03-19 06:10'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Decompilation revealed two OAuth2 client IDs for different security levels: step-level-2 (scope 'aula') and step-level-3 (scope 'aula-sensitive'). The SimpleSAMLphp OIDC flow uses login.aula.dk. Worth investigating what triggers step-up auth and what endpoints require it (e.g., sensitive messaging, secure documents). Client IDs: _742adb5e2759028d86dbadf4af44ef70e8b1f407a6 and _99949a54b8b65423862aac1bf629599ed64231607a.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Document both OAuth2 client IDs, their scopes, and security levels
- [x] #2 Identify what triggers step-up authentication (level 2 to level 3)
- [x] #3 Map which endpoints or operations require the sensitive/elevated scope
- [x] #4 Document the step-up authentication flow sequence
- [x] #5 Identify the LoginAuthenticationMethod enum values and their meanings
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read and trace all step-up related code (done)
2. Add comprehensive step-up auth section to auth_flow.md
3. Document: triggers, affected operations, flow sequence, error handling
4. Check for justfile and run e2e tests
5. Commit and mark task done
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Documented step-up authentication flow in auth_flow.md section 14.

Key findings:
- Two OAuth2 client IDs map to different auth levels: Level 2 (_742adb5e..., scope "aula") for UniLogin, Level 3 (_99949a54..., scope "aula-sensitive") for MitID/NemID
- Guardians and employees are forced to step up if they log in at Level 2 (effectively always need MitID)
- Children >=15 get optional step-up suggestion (mandatory if pending onboarding)
- Children <15 stay at Level 2 with no prompt
- Operations requiring step-up: secure documents, sensitive messages (mark/view), consent editing, Aula document link rendering
- Server signals step-up via HTTP 401 + sub-code 8 (AUTHORIZATION_STEP_UP_REQUIRED)
- Step-up flow: logout, re-auth with Level 3 client ID, SimpleSAMLphp presents MitID instead of UniLogin
- LoginAuthenticationMethod enum: Unknown=0, Level2=2, Level3NemId=3, Level3Employees=9
- SensitivityLevel enum on messages: Level1=1, Level2=2, Level3=3. Setting sensitive = SensitivityLevel 3.
- Permission model includes per-permission StepUp boolean from backend

Added 17 step-up-related source files to the appendix index.
<!-- SECTION:FINAL_SUMMARY:END -->
