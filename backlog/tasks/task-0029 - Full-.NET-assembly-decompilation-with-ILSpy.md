---
id: TASK-0029
title: Full .NET assembly decompilation with ILSpy
status: To Do
assignee: []
created_date: '2026-03-18 15:07'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Use ILSpy/ilspycmd to fully decompile the extracted AulaNative.dll (assembly_187, 4.3MB) to get complete C# source code. This will reveal actual URL path strings, HTTP method assignments, request/response body schemas, and header configurations that could not be extracted via string analysis alone. The assembly has been extracted and LZ4-decompressed to extracted_assemblies/assembly_187_AulaNative.dll. May require adding ilspycmd or dotnet SDK to shell.nix.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 ILSpy or equivalent decompiler successfully runs on extracted assembly
- [ ] #2 Complete C# source code for all AulaNative.Services.Web.* service classes
- [ ] #3 All REST endpoint URL paths confirmed (not just inferred from method names)
- [ ] #4 HTTP method assignments verified for each endpoint
- [ ] #5 Request/response JSON schemas documented
<!-- AC:END -->
