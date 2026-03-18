---
id: TASK-0043
title: Profile and institution model types
status: To Do
assignee: []
created_date: '2026-03-18 16:08'
labels:
  - rust
  - aula-api
  - models
dependencies:
  - TASK-0042
references:
  - data_models.md
  - domain_concepts.md
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Rust structs for the identity/organizational domain: Profile, InstitutionProfile, ChildProfile, RelationProfile, User, Institution, Municipality, AdministrativeAuthority, Address, Permission, PermissionEnum. InstitutionProfileId is the pivot entity for nearly all API operations (see domain_concepts.md Section 5.1). Includes the layered identity model: User -> Profile -> InstitutionProfile[]. See data_models.md Models.ProfileModels and Models.Institutions namespaces.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Profile, InstitutionProfile, ChildProfile, RelationProfile structs with serde Deserialize
- [ ] #2 Institution, Municipality, AdministrativeAuthority structs
- [ ] #3 Permission struct and PermissionEnum with 100+ variants
- [ ] #4 Address, ContactInfo, and related value types
- [ ] #5 Type aliases for InstitutionProfileId, ProfileId, InstitutionCode as domain-specific newtypes or type aliases
<!-- AC:END -->
