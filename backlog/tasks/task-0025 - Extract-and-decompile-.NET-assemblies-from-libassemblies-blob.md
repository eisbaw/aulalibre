---
id: TASK-0025
title: Extract and decompile .NET assemblies from libassemblies blob
status: To Do
assignee: []
created_date: '2026-03-18 14:35'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The business logic is packed in libassemblies.x86_64.blob.so. Need to decompress the Xamarin/MAUI assembly blob to extract individual DLLs (AulaNative.dll, AulaNative.Droid.dll, etc.) and decompile them with ILSpy or similar. This will reveal API endpoints, feature flags, authentication flow details, and certificate pinning implementation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Assembly blob extracted to individual DLL files
- [ ] #2 DLLs decompiled to readable C# source
- [ ] #3 API endpoints and base URLs catalogued from decompiled source
<!-- AC:END -->
