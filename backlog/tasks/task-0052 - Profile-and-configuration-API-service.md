---
id: TASK-0052
title: Profile and configuration API service
status: To Do
assignee: []
created_date: '2026-03-18 16:10'
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
- [ ] #1 Profile service: get profiles by login, get/update master data, update profile picture
- [ ] #2 Configuration service: max file size, authorized formats, app deprecated check, privacy policy
- [ ] #3 Additional master data service: get/update by institution profile
- [ ] #4 All methods return properly typed Results with domain model types
<!-- AC:END -->
