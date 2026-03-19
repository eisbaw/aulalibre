---
id: TASK-0016
title: Extract and analyze protobuf definitions
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:59'
updated_date: '2026-03-18 23:48'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Found .proto files in the base APK: client_analytics.proto, messaging_event.proto, messaging_event_extension.proto. These define the wire format for analytics and messaging. Extract and document the protobuf schemas to understand the app's data model and communication protocols.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Locate all .proto files in extracted APK contents
- [x] #2 Extract and document protobuf schema definitions (fields, types, nesting)
- [x] #3 Search decompiled C# for Google.Protobuf or protobuf-net usage patterns
- [x] #4 Search jadx output for protobuf-related classes and generated code
- [x] #5 Determine if DataStore uses protobuf serialization
- [x] #6 Document findings in a protobuf analysis file
- [x] #7 Create follow-up tasks for any new tangents discovered
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Locate all .proto files in extracted APK
2. Read and document each proto schema
3. Search decompiled C# for protobuf usage (Google.Protobuf, protobuf-net)
4. Search jadx output for protobuf classes and generated code
5. Check DataStore protobuf usage
6. Identify whether protobuf is Aula-specific or only library/Firebase
7. Write analysis document
8. Check ACs, add notes, final summary
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Found 3 .proto files in APK root: client_analytics.proto, messaging_event.proto, messaging_event_extension.proto
- All are Google/Firebase SDK protos (Copyright Google LLC), not Aula-specific
- Protobuf runtimes in DEX: androidx.datastore.preferences.protobuf, com.google.crypto.tink.shaded.protobuf, kotlinx.serialization.protobuf
- Zero protobuf usage in Aula C# application code -- app uses Newtonsoft.Json throughout
- No com.netcompany packages in DEX files (expected for Xamarin)
- kotlinx.serialization.protobuf annotations present but unused (transitive dependency)
- Created 3 follow-up tasks: TASK-0080 (Firebase analytics), TASK-0081 (Tink crypto), TASK-0082 (DataStore prefs)
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed all protobuf definitions and usage in the Aula APK.

Findings:
- 3 .proto files found in APK root are all Google/Firebase SDK schemas (client_analytics.proto for Firebase Data Transport metrics, messaging_event.proto and messaging_event_extension.proto for FCM event reporting). None are Aula-specific.
- Protobuf runtimes in DEX serve infrastructure only: AndroidX DataStore preference serialization, Firebase analytics transport, and Google Tink crypto key formats.
- The Aula application code (C#/.NET) uses Newtonsoft.Json for all serialization -- zero protobuf usage in application logic.
- kotlinx.serialization.protobuf annotations are a transitive dependency with no actual usage.

Files:
- analysis/protobuf_analysis.md -- full schema documentation and usage analysis

Follow-up tasks created: TASK-0080 (Firebase analytics data collection), TASK-0081 (Tink crypto key usage), TASK-0082 (DataStore preference keys).
<!-- SECTION:FINAL_SUMMARY:END -->
