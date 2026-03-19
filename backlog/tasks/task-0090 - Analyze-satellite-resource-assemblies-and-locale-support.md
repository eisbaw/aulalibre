---
id: TASK-0090
title: Analyze satellite resource assemblies and locale support
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-19 07:14'
updated_date: '2026-03-19 07:49'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The blob contains 12 locale variants of Microsoft.VisualStudio.Validation.resources (indices 175-186). This pattern may apply to other assemblies too. Investigate which locales Aula supports, whether Danish locale resources contain translatable strings useful for understanding the app's feature set, and whether any locale-specific logic exists.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Identify all satellite resource assemblies and their locale codes
- [x] #2 Determine whether Aula has its own localization resources or relies on framework-only
- [x] #3 Document which languages are supported at each layer (app vs framework)
- [x] #4 Document the I18NPortable localization architecture and key statistics
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Identify satellite resource assemblies and extract locale codes from binaries
2. Analyze I18NPortable localization framework usage in Aula code
3. Map the dk.json locale file structure and key statistics
4. Check Android resource layer for locale support
5. Write analysis document with findings
6. Commit and mark task done
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed all satellite resource assemblies and locale support in the Aula APK.

Findings:
- 13 satellite resource assemblies exist, all belonging to Microsoft.VisualStudio.Validation (cs, de, es, fr, it, ja, ko, pl, pt-BR, ru, tr, zh-Hans, zh-Hant) -- framework artifacts, not Aula content
- Aula uses I18NPortable with a single Danish locale file (dk.json) containing 2,189 translation keys
- The app supports exactly one language: Danish (da-DK)
- Danish culture is hardcoded beyond UI strings (date formatting, string comparison, sorting)
- 7 translation keys are referenced but missing from dk.json
- Android resource layer contains only framework strings, no Aula-specific localized resources

Deliverable: locale_analysis.md
<!-- SECTION:FINAL_SUMMARY:END -->
