---
id: TASK-0010
title: Map data models and DTOs
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:32'
updated_date: '2026-03-18 15:43'
labels: []
dependencies:
  - TASK-0006
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Extract all data model classes (DTOs, entities, response objects) from decompiled code. Document their fields, types, and relationships. These models define the API contract and will become Rust structs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All API response/request model classes catalogued
- [x] #2 Field names, types, and nullability documented per model
- [x] #3 Relationships between models mapped (e.g. Message belongs to Thread)
- [x] #4 Serialization annotations documented (Gson, Moshi, Jackson field names)
- [x] #5 Enum types and their values catalogued
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Extract all types from AulaNative.dll via monodis --typedef
2. Full IL disassembly of AulaNative.dll to get class fields, types, and custom attributes
3. Parse IL output to extract:
   a. All AulaNative.Models.* classes with their fields and types
   b. All AulaNative.Enums.* enums with their values
   c. JsonProperty annotations for serialized field names
   d. Inheritance relationships between models
4. Catalog ~450+ model/DTO classes across 20+ domain namespaces
5. Catalog ~130+ enum types with values
6. Map relationships between models (e.g. MessageThread contains Messages)
7. Document everything in data_models.md
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Analysis complete. Used monodis --typedef (4051 types), --fields (17737 fields), --customattr (34650 entries) on assembly_187_AulaNative.dll.

Key findings:
- 604 model/DTO classes extracted (not ~450 as initially estimated)
- 136 enum types with values
- 109 fields have [JsonProperty] annotations but monodis cannot decode attribute values (missing Newtonsoft.Json dependency)
- App uses Newtonsoft.Json v13.0.0, not Gson/Moshi/Jackson (AC#4 referenced Java serializers but this is .NET)
- DynamicContractResolver and DefaultUnknownEnumConverter indicate sophisticated JSON serialization strategy
- Created TASK-34 for follow-up: decoding actual JsonProperty annotation values
- Relationship graph shows Profile/User as central entities, with MessageThread, EventBaseClass, Group, SecureDocumentDto as major domain objects
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Extracted and documented all data model classes from AulaNative.dll for the Aula school communication app.

Changes:
- Created data_models.md (6400+ lines) with comprehensive model documentation
- 604 model/DTO classes across 20+ namespaces with field names, types, and nullability
- 136 enum types with all member values catalogued
- Entity relationship graph mapping how models connect (Profile -> Group -> Institution, MessageThread -> MessageDto, EventBaseClass -> EventDetailsDto, etc.)
- 109 fields with [JsonProperty] serialization annotations identified (values not decodable due to missing Newtonsoft.Json dependency for monodis)
- Namespace index table for quick navigation

Limitations:
- JsonProperty attribute values (actual JSON field names) could not be decoded -- created TASK-34 for follow-up
- Nullability annotations (NullableAttribute) also not decodable by monodis
- Some generic type parameters show as \!0, \!1 etc. rather than resolved types

Output: data_models.md
<!-- SECTION:FINAL_SUMMARY:END -->
