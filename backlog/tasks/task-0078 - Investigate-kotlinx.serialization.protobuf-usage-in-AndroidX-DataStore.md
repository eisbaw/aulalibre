---
id: TASK-0078
title: Investigate kotlinx.serialization.protobuf usage in AndroidX DataStore
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-18 23:42'
updated_date: '2026-03-19 06:26'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
AndroidX DataStore v1.1.7 bundles kotlinx.serialization.protobuf. DataStore is used for local preferences/settings storage. Investigate what data Aula stores in DataStore and whether the protobuf serializer is used for typed settings objects. This could reveal app configuration structure and stored preferences schema.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Determine whether kotlinx.serialization.protobuf is actually present in the APK
- [x] #2 Identify all DataStore usage (app-specific vs library-internal)
- [x] #3 Catalog the bundled .proto files and determine their origin
- [x] #4 Document the protobuf serialization schema used by DataStore Preferences
- [x] #5 Document findings in an analysis file
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Search DEX code for kotlinx.serialization.protobuf - determine if it exists
2. Trace all DataStore usage in both DEX files
3. Examine bundled .proto files
4. Analyze PreferencesProto schema
5. Write findings document
6. Commit
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Investigated kotlinx.serialization.protobuf usage in the Aula APK. Key finding: kotlinx.serialization.protobuf is NOT present in the app. The protobuf usage comes entirely from two standard library sources:

1. AndroidX DataStore Preferences (v1.1.7) uses protobuf-lite internally (repackaged under androidx.datastore.preferences.protobuf) to serialize generic key-value pairs in .preferences_pb files. Only Firebase HeartBeatInfoStorage uses it.

2. Three bundled .proto files (client_analytics.proto, messaging_event.proto, messaging_event_extension.proto) are standard Firebase/Google transport definitions, not Aula-specific.

No Aula application code interacts with DataStore directly. All DataStore usage is Firebase library internals for heartbeat tracking. The reconstructed PreferencesProto schema and all findings are documented in analysis/datastore_protobuf_analysis.md.
<!-- SECTION:FINAL_SUMMARY:END -->
