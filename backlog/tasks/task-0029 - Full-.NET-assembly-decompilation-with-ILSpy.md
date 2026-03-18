---
id: TASK-0029
title: Full .NET assembly decompilation with ILSpy
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 15:07'
updated_date: '2026-03-18 22:00'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Use ILSpy/ilspycmd to fully decompile the extracted AulaNative.dll (assembly_187, 4.3MB) to get complete C# source code. This will reveal actual URL path strings, HTTP method assignments, request/response body schemas, and header configurations that could not be extracted via string analysis alone. The assembly has been extracted and LZ4-decompressed to extracted_assemblies/assembly_187_AulaNative.dll. May require adding ilspycmd or dotnet SDK to shell.nix.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ILSpy or equivalent decompiler successfully runs on extracted assembly
- [x] #2 Complete C# source code for all AulaNative.Services.Web.* service classes
- [x] #3 All REST endpoint URL paths confirmed (not just inferred from method names)
- [x] #4 HTTP method assignments verified for each endpoint
- [x] #5 Request/response JSON schemas documented
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add ilspycmd to shell.nix
2. Decompile AulaNative.dll (assembly_187) to C# source using ilspycmd
3. Decompile AulaNative.Droid.dll and AulaNative.Droid.Private.dll if present
4. Organize output in decompiled_csharp/ directory, gitignored
5. Extract and document: URL paths, HTTP methods, JsonProperty annotations, request/response schemas
6. Create follow-up tasks for interesting findings
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- ilspycmd 9.1 added to shell.nix and works correctly
- Decompiled assembly_187_AulaNative.dll to 1825 C# files in decompiled_csharp/AulaNative/
- Decompiled assembly_0_AulaNative.Droid.dll to 778 C# files in decompiled_csharp/AulaNative.Droid/
- Found Urls.cs with ALL API endpoints using ?method= query parameter routing
- Found Conf.cs with environment configs, OAuth client IDs, cert pinning keys
- Found SimpleService.cs base class confirming GET/POST HTTP method assignments
- Created decompilation_analysis.md with complete endpoint map
- decompiled_csharp/ added to .gitignore
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Successfully decompiled AulaNative.dll (4.3MB) and AulaNative.Droid.dll using ILSpy (ilspycmd 9.1) to full C# source code.

Key deliverables:
- 1825 C# source files from AulaNative.dll in decompiled_csharp/AulaNative/
- 778 C# source files from AulaNative.Droid.dll in decompiled_csharp/AulaNative.Droid/
- Comprehensive decompilation_analysis.md with:
  - Complete REST API endpoint map (170+ endpoints across 15 API modules)
  - All URL paths confirmed via Urls.cs (uses ?method= query parameter routing, NOT REST paths)
  - HTTP method assignments verified from service class decompilation (Get vs Post/SimplePost)
  - Request/response JSON schemas for key endpoints (messaging, posts, calendar)
  - OAuth2 authentication details (client IDs, scopes, SimpleSAMLphp OIDC endpoints)
  - Certificate pinning keys for aula.dk and ncaula.com
  - Environment configuration for prod, preprod, and 10+ test environments

Key architectural finding: The API is RPC-style, not RESTful. All endpoints route through a single URL base with ?method=module.action query parameters.

Created follow-up tasks: TASK-71 (OAuth2 step-up auth), TASK-72 (API client), TASK-73 (cert pinning).
<!-- SECTION:FINAL_SUMMARY:END -->
