---
id: TASK-0043
title: Profile and institution model types
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:08'
updated_date: '2026-03-18 17:25'
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
- [x] #1 Profile, InstitutionProfile, ChildProfile, RelationProfile structs with serde Deserialize
- [x] #2 Institution, Municipality, AdministrativeAuthority structs
- [x] #3 Permission struct and PermissionEnum with 100+ variants
- [x] #4 Address, ContactInfo, and related value types
- [x] #5 Type aliases for InstitutionProfileId, ProfileId, InstitutionCode as domain-specific newtypes or type aliases
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create models/ module directory under aula-api/src/
2. Create models/mod.rs with sub-module declarations
3. Create models/profiles.rs with core profile types:
   - Type aliases: InstitutionProfileId, ProfileId, InstitutionCode
   - Address, ProfilePictureDto, BlockedCommunicationInfo
   - InstitutionProfileBase (fields shared across profile variants)
   - InstitutionProfile, InstitutionProfileChild, ChildProfile, RelationProfile
   - Profile (top-level with groups, phone numbers, etc.)
   - ProfileContext, StubbedUser, SimpleInstitutionProfile
4. Create models/institutions.rs with:
   - AdministrativeAuthority, InstitutionIdentity, SimpleInstitution
   - Institution (full model with children, permissions, groups)
   - Permission struct referencing PermissionEnum
5. Create models/groups.rs with:
   - Group, GroupMembership, StubbedGroup, GroupMemberGroup
   - SimpleGroupDto, RecipientRelation, MainGroup
6. Create models/users.rs with:
   - User, UserRelationship, ProfileContext
7. Register models module in lib.rs and re-export key types
8. Run nix-shell --run just e2e to verify
9. Check all ACs and mark done
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented 4 model files under aula-api/src/models/:
- profiles.rs: 20+ structs covering the full profile hierarchy
- institutions.rs: Institution, AdministrativeAuthority, InstitutionIdentity, SimpleInstitution
- groups.rs: Group, GroupMembership, RecipientRelation, SimpleGroupDto, etc.
- users.rs: User, ProfileContext, UserRelationship

All fields use Option<T> for nullable API fields. serde(rename_all = "camelCase") on all structs.
Permission struct references PermissionEnum from TASK-42. Type aliases for InstitutionProfileId, ProfileId, InstitutionCode.
179 tests pass including 16 new deserialization tests. Clippy clean, fmt clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added profile and institution domain model types to aula-api, covering the Aula identity hierarchy (User -> Profile -> InstitutionProfile).

Changes:
- New models/ module with 4 sub-modules: profiles.rs, institutions.rs, groups.rs, users.rs
- profiles.rs: Profile, InstitutionProfile, InstitutionProfileBase, InstitutionProfileChild, ChildProfile, RelationProfile, ChildRelationsProfile, Permission, Address, ProfilePictureDto, AulaFileContent, BlockedCommunicationInfo, SimpleInstitutionProfile, StubbedUser, ChildMetadata, EmployeeMetadata, ComeGoInstitutionProfile, ContactListInstitutionProfile, PageConfiguration, WidgetDto, ModuleDto, and supporting types
- institutions.rs: Institution, AdministrativeAuthority, InstitutionIdentity, SimpleInstitution
- groups.rs: Group, GroupMembership, GroupMembershipInstitutionProfile, RecipientRelation, SimpleGroupDto, StubbedGroup, GroupMemberGroup, GroupModule, GroupWidget, GroupByContextDto
- users.rs: User, ProfileContext, UserRelationship
- Type aliases: InstitutionProfileId (i64), ProfileId (i64), InstitutionCode (String)
- All structs derive Debug, Clone, Serialize, Deserialize with serde(rename_all = "camelCase")
- References PermissionEnum and profile enums from TASK-42

Tests:
- 16 new deserialization tests covering all major types
- All 179 tests pass, clippy clean, fmt clean
<!-- SECTION:FINAL_SUMMARY:END -->
