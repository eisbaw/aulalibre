---
id: TASK-0052
title: Profile and configuration API service
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:10'
updated_date: '2026-03-18 18:32'
labels:
  - rust
  - aula-api
  - service
dependencies:
  - TASK-0049
  - TASK-0043
references:
  - api_endpoints.md
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the profile and configuration API service module. Profiles: GetProfilesByLogin (post-auth profile fetch), GetProfileMasterData, GetOnboardingMasterData, PostMasterData, PostUpdateProfilePicture, KeepAlive. Configuration: GetMaxFileSize, GetAuthorizedFileFormats, IsAppDeprecated, GetPrivacyPolicy, GetLoginImportantInformation. Additional master data: GetAdditionalMasterData, GetAdditionalMasterDataByInstitutionProfileId, PostAdditionalMasterData. See api_endpoints.md Sections 3.2-3.4.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Profile service: get profiles by login, get/update master data, update profile picture
- [x] #2 Configuration service: max file size, authorized formats, app deprecated check, privacy policy
- [x] #3 Additional master data service: get/update by institution profile
- [x] #4 All methods return properly typed Results with domain model types
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create services/ module directory under aula/aula-api/src/
2. Create services/mod.rs exporting profile, configuration, additional_master_data submodules
3. Create services/profiles.rs: get_profiles_by_login, get_profile_master_data, get_onboarding_master_data, post_master_data, update_profile_picture, keep_alive
4. Create services/configuration.rs: get_max_file_size, get_authorized_file_formats, is_app_deprecated, get_privacy_policy, get_login_important_information, get_administrative_authority
5. Create services/additional_master_data.rs: get_additional_master_data, get_by_institution_profile_id, post_additional_master_data, post_additional_master_data_employee
6. Register services module in lib.rs
7. All methods take &mut Session and return Result<T> using existing model types
8. Run just e2e
9. Mark ACs complete
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented profile and configuration API service modules for aula-api.

Changes:
- Created `services/` module directory with `mod.rs`, `profiles.rs`, `configuration.rs`, `additional_master_data.rs`
- Profile service: `get_profiles_by_login`, `get_profile_master_data`, `get_onboarding_master_data`, `post_master_data`, `update_profile_picture`, `keep_alive` -- all taking `&mut Session` and returning `Result<T>` with domain model types
- Configuration service: `get_max_file_size`, `get_authorized_file_formats`, `is_app_deprecated`, `get_privacy_policy`, `get_administrative_authority`, `get_login_important_information`
- Additional master data service: `get_additional_master_data`, `get_by_institution_profile_id`, `post_additional_master_data`, `post_additional_master_data_employee`
- Registered `pub mod services` in lib.rs
- All endpoint paths documented as inferred (not verified against live traffic) per api_endpoints.md Sections 3.2-3.4
- Response types use existing model types where available; new DTOs created with serde_json::Value fallback for unknown response shapes
- 13 unit tests for request/response serialization

Tests: `just e2e` passes (350 tests, clippy clean, fmt clean)
<!-- SECTION:FINAL_SUMMARY:END -->
