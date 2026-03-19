---
id: TASK-0093
title: Fix ProfilesByLoginResponse model to match OnboardingResponseDto
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 15:50'
updated_date: '2026-03-19 16:25'
labels:
  - rust-cli
  - api-model
  - blocking
dependencies:
  - TASK-0092
references:
  - >-
    decompiled_csharp/AulaNative/AulaNative.DTOs.Onboarding/OnboardingResponseDto.cs
  - >-
    decompiled_csharp/AulaNative/AulaNative.DTOs.Onboarding/OnboardingProfileDto.cs
  - >-
    decompiled_csharp/AulaNative/AulaNative.DTOs.Onboarding/StubbedInstitutionProfile.cs
  - decompiled_csharp/AulaNative/AulaNative.DTOs.Onboarding/StubbedChild.cs
  - >-
    decompiled_csharp/AulaNative/AulaNative.Models.ProfileModels/InstitutionProfileBase.cs
  - aula/aula-api/src/services/profiles.rs
  - aula/aula-api/src/models/profiles.rs
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The getprofilesbylogin endpoint returns OnboardingResponseDto, not our current Profile model. The real response contains OnboardingProfileDto with fields: portalRole, isLatestDataPolicyAccepted, children (list of StubbedChild with InstitutionProfileChild), institutionProfiles (list of StubbedInstitutionProfile with id/profileId), overConsentAge, contactInfoEditable, profileId, displayName. Our current Profile struct maps none of these correctly - all fields deserialize as null. This is the root cause of empty profile display and blocks extracting institutionProfileId needed by most other API endpoints.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ProfilesByLoginResponse model matches real API OnboardingResponseDto structure
- [x] #2 InstitutionProfileId extracted and available for use by other commands
- [x] #3 Children profiles parsed with names, institution codes, profile IDs
- [x] #4 Guardian institutionProfiles parsed with id, profileId, names, addresses
- [x] #5 profile me command displays real name, institution, children
- [x] #6 Existing tests updated to match new model structure
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Rewrote ProfilesByLoginResponse to match real OnboardingResponseDto from API. Added LoginInstitutionProfile, LoginChild, LoginChildInstitutionProfile models. profile me now displays full guardian and children data. All tests pass, verified against production.
<!-- SECTION:FINAL_SUMMARY:END -->
