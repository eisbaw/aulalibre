---
id: TASK-0024
title: Decompile CRC64 app classes with jadx for detailed analysis
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-18 14:14'
updated_date: '2026-03-19 05:35'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The 678 CRC64 classes in classes2.dex are Android Callable Wrappers for the Aula .NET code. Decompiling them with jadx would reveal: activity lifecycle implementations, view binding patterns, intent extras/data passing between screens, WebView JavaScript bridge interfaces, and Firebase messaging service implementation. Focus on classes2.dex only.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 classes2.dex decompiled with jadx to readable Java source
- [ ] #2 Key activity lifecycle patterns documented (what data is passed between screens via intents)
- [ ] #3 WebView JavaScript bridge interfaces identified if any
- [ ] #4 Firebase messaging service implementation analyzed
- [ ] #5 Findings documented in a committed analysis file
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Run jadx on classes2.dex to decompile to Java source\n2. Survey the decompiled output structure\n3. Analyze Activity classes for lifecycle patterns and intent data passing\n4. Search for WebView setup and JavaScript bridge interfaces\n5. Analyze AulaFirebaseMessagingService implementation\n6. Cross-reference with CRC64 namespace mapping and decompiled C# source\n7. Document findings\n8. Commit and finalize task
<!-- SECTION:PLAN:END -->
