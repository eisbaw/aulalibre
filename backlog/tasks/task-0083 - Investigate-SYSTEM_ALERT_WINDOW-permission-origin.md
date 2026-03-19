---
id: TASK-0083
title: Investigate SYSTEM_ALERT_WINDOW permission origin
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 23:55'
updated_date: '2026-03-19 07:21'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
SYSTEM_ALERT_WINDOW is declared in AndroidManifest.xml but no usage found in decompiled C# code. Investigate whether this is a Xamarin framework artifact, a dependency requirement, or truly unused. If unused, document as a recommendation for removal.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Search all extracted APK artifacts (manifests, smali, Java, C#) for SYSTEM_ALERT_WINDOW references
- [x] #2 Identify which component (app manifest, library manifest, or merged manifest) declares the permission
- [x] #3 Determine root cause: Xamarin framework artifact, library dependency, or app-declared
- [x] #4 Document findings in security_analysis.md or architecture docs
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Investigated the origin of SYSTEM_ALERT_WINDOW permission declared in AndroidManifest.xml.

Findings:
- Searched all extracted artifacts: decompiled C#, smali (classes.dex + classes2.dex), jadx Java output, and all AndroidManifest.xml files across APK splits
- Zero usage found: no canDrawOverlays(), no TYPE_APPLICATION_OVERLAY, no TYPE_SYSTEM_ALERT references anywhere
- The permission sits in the app-declared block (lines 3-12) of the merged manifest, separate from library-merged permissions (lines 43-46 where AndroidX/Firebase contributions appear)
- Ruled out Xamarin framework: Microsoft docs confirm Xamarin only auto-adds INTERNET and READ_EXTERNAL_STORAGE in debug builds
- Ruled out Firebase/Google Play Services: their manifest contributions are in a distinct block and do not include this permission

Conclusion: SYSTEM_ALERT_WINDOW was explicitly declared by the Aula app developer. It is completely unused and should be removed to reduce the app attack surface.

Updated security_analysis.md permission table (row 12) and security concerns table (row 3) with definitive findings.
<!-- SECTION:FINAL_SUMMARY:END -->
