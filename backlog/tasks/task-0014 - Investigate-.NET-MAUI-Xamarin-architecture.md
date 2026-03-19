---
id: TASK-0014
title: Investigate .NET MAUI/Xamarin architecture
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:59'
updated_date: '2026-03-18 23:32'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Aula app uses Mono/.NET runtime (libmonodroid.so, libmonosgen-2.0.so, libassemblies.x86_64.blob.so). This means the app logic is in .NET assemblies, not Java/Kotlin DEX code. The assemblies blob needs to be extracted and decompiled with .NET tools (ILSpy/dnSpy) rather than jadx. This fundamentally changes the reverse engineering approach.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Document the DI container setup (Unity container, service registrations, ServiceLocator pattern)
- [x] #2 Document the layered architecture (Services.Web -> ServiceManagers -> ViewModels -> Activities)
- [x] #3 Document the navigation pattern (Android Activities/Fragments, tab-based main page)
- [x] #4 Document singleton managers and cross-platform service abstractions
- [x] #5 Document the data flow: API calls, AutoMapper, models/DTOs, SQLite local storage
- [x] #6 Produce architecture.md with findings
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Analyze DI container setup in AndroidDependencyInjectionConfig + CommonDependencyInjectionConfig
2. Map the layered architecture from decompiled source structure
3. Trace navigation pattern from MainApplication -> Activities -> Fragments
4. Catalog singleton managers and their responsibilities
5. Document data flow from API through service layer to ViewModels
6. Write architecture.md with comprehensive findings
7. Create follow-up backlog tasks for tangents discovered
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Analyzed AndroidDependencyInjectionConfig.cs and CommonDependencyInjectionConfig.cs: Unity Container + CommonServiceLocator pattern with 35+ registered services
- Mapped 5-layer architecture: Activities(453) -> ViewModels(520) -> ServiceManagers(30) -> Services.Web(31) -> HTTP/SimpleService
- Found DomainService layer between ServiceManagers and ViewModels for complex business logic
- Discovered MessagingCenter event bus (25 domain-specific centers) for cross-component communication
- Documented 11 singleton managers with manual lifecycle management
- Identified OIDC auth with step-up authentication (two scope levels)
- Found server-driven module/widget configuration system controlling bottom tab bar
- Created 4 follow-up tasks: TASK-74 (OIDC auth), TASK-75 (MessagingCenter), TASK-76 (module config), TASK-77 (security)
- Produced architecture.md with comprehensive findings
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Investigated and documented the .NET MAUI/Xamarin architecture of the Aula Android app from decompiled C# source.

Findings:
- Xamarin.Android app (not Xamarin.Forms), using native Activities/Fragments with Unity DI container
- 5-layer architecture: Activities(453) -> ViewModels(520) -> ServiceManagers(30) -> Services.Web(31) -> HTTP infrastructure
- DomainService layer for complex business orchestration across multiple services
- 11 singleton managers for profile, HTTP, session, files, encryption, SQLite, etc.
- OIDC authentication via login.aula.dk with step-up auth (aula vs aula-sensitive scopes)
- Server-driven module/widget system controlling available features per user/institution
- 25 MessagingCenter event bus classes for decoupled component communication
- Certificate pinning, CSRF protection, encrypted local storage

Produced: architecture.md
Created follow-up tasks: TASK-74, TASK-75, TASK-76, TASK-77
<!-- SECTION:FINAL_SUMMARY:END -->
