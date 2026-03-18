---
id: TASK-0010
title: Map data models and DTOs
status: To Do
assignee: []
created_date: '2026-03-18 13:32'
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
- [ ] #1 All API response/request model classes catalogued
- [ ] #2 Field names, types, and nullability documented per model
- [ ] #3 Relationships between models mapped (e.g. Message belongs to Thread)
- [ ] #4 Serialization annotations documented (Gson, Moshi, Jackson field names)
- [ ] #5 Enum types and their values catalogued
<!-- AC:END -->
