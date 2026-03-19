---
id: TASK-0089
title: >-
  Investigate AulaNative.Droid.Private assembly - tiny 5KB DLL with possible
  sensitive logic
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-19 07:14'
updated_date: '2026-03-19 07:16'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The AulaNative.Droid.Private assembly is only 5KB (2 .cs files after decompilation) but its name suggests it may contain private/sensitive app configuration, certificate pinning overrides, or debug flags. Worth investigating what exactly it contains and whether it has security implications.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Identify all code in AulaNative.Droid.Private assembly
- [x] #2 Determine why it exists as a separate assembly from AulaNative.Droid
- [x] #3 Check for sensitive logic, API keys, or private configurations
- [x] #4 Document findings in implementation notes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read all decompiled .cs files in AulaNative.Droid.Private
2. Compare with other assemblies that have the same Resource.cs pattern
3. Check for any sensitive content (keys, certs, configs)
4. Determine the purpose of the "Private" suffix
5. Document findings
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Investigation Results

The AulaNative.Droid.Private assembly contains exactly 2 source files after decompilation:

### 1. Resource.cs (AulaNative.Droid.Private namespace)
An empty class that inherits from _Microsoft.Android.Resource.Designer.Resource. This is a standard .NET MAUI/Android boilerplate -- every Android library project gets one. It provides compile-time access to Android resource IDs (layouts, strings, drawables). The class body is empty.

### 2. AssemblyInfo.cs (Properties/)
Standard assembly metadata:
- Title: AulaNative.Droid.Private
- Version: 1.0.0.0
- Git commit hash in informational version: 23b75ce850aed1d8bbdfcb63cdb6b2cfc239b3a4
- Target: Android35 (min Android29.0)
- Copyright template variable: ${AuthorCopyright} (unresolved -- build-time substitution)

### Why "Private"?
The "Private" suffix refers to the app VARIANT, not to secret/sensitive code. The Android package name is com.netcompany.aulanativeprivate -- this is the private/consumer variant of Aula (as opposed to a potential public/institutional variant). In .NET MAUI, each app variant that needs its own Android resources (app icon, splash screen, package-specific resource IDs) gets a separate library project. AulaNative.Droid.Private exists solely to provide the resource bridge for the "private" variant build.

### Security Assessment
- NO API keys, tokens, or credentials found
- NO certificate pinning configuration
- NO debug flags or feature toggles
- NO sensitive logic whatsoever
- The assembly is purely structural build infrastructure

### Comparison
The same Resource.cs pattern appears in Plugin.Fingerprint, MonkeyCache, SQLitePCLRaw, and other library projects -- all with identical empty Resource class stubs.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
AulaNative.Droid.Private is a build-infrastructure-only assembly with zero sensitive logic. It contains:

1. An empty Resource.cs class inheriting from the .NET MAUI Android resource designer (standard boilerplate for every Android library project)
2. A standard AssemblyInfo.cs with version metadata

The "Private" suffix refers to the app variant (com.netcompany.aulanativeprivate -- the consumer/parent-facing Aula app), not to secret or private code. The assembly exists to provide variant-specific Android resource ID bindings during build. This is the same pattern used by Plugin.Fingerprint, MonkeyCache, SQLitePCLRaw, and other library projects in the solution.

No API keys, certificates, debug flags, or sensitive configuration found.
<!-- SECTION:FINAL_SUMMARY:END -->
