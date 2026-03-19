---
id: TASK-0028
title: Decompile .NET assemblies (DLLs) from APK
status: To Do
assignee: []
created_date: '2026-03-18 14:43'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The DEX decompilation (TASK-6) confirmed that Aula is a .NET MAUI/Xamarin app where the DEX layer is just Android Callable Wrappers. The actual business logic, API clients, data models, and auth token handling live in .NET assemblies (DLLs) embedded in the APK. These need to be extracted and decompiled with a .NET decompiler (ilspycmd, dotPeek, or dnSpy) to access the real application code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Locate .NET DLL assemblies in the extracted APK
- [ ] #2 Decompile DLLs to C# source using ilspycmd or equivalent
- [ ] #3 Identify API client classes and endpoint URLs
- [ ] #4 Document data models and auth flow
<!-- AC:END -->
