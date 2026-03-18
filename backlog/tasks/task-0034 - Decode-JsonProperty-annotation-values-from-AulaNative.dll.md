---
id: TASK-0034
title: Decode JsonProperty annotation values from AulaNative.dll
status: To Do
assignee: []
created_date: '2026-03-18 15:42'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
monodis cannot decode Newtonsoft.Json attributes because the dependency assembly is missing. 109 fields have [JsonProperty] annotations whose actual JSON field names are unknown. Options: (1) provide Newtonsoft.Json.dll in the same directory as the assembly for monodis, (2) use ilspycmd or dnSpy to get full decompilation, (3) capture actual network traffic to discover field names. This is important for generating correct Rust serde annotations.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Actual JSON field names documented for all 109 annotated properties
- [ ] #2 Serde rename annotations prepared for Rust struct generation
<!-- AC:END -->
