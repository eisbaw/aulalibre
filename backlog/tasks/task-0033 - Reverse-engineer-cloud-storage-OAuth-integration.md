---
id: TASK-0033
title: Reverse engineer cloud storage OAuth integration
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 15:24'
updated_date: '2026-03-19 05:50'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Separate OAuth2 flows for Google Drive and OneDrive integration. Uses CloudStorageAuthenticatorManager with platform-specific client IDs. Paths: /googleoauth2redirect and /onedrive2redirect.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Cloud storage provider abstraction layer documented (interfaces, implementations)
- [x] #2 File upload/download flow from cloud storage mapped end-to-end
- [x] #3 OAuth scopes and permissions for each provider documented
- [x] #4 How cloud files are attached to Aula messages/posts documented
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read all cloud storage related decompiled classes
2. Map the provider abstraction layer
3. Map the file browse/import/attach flow end-to-end
4. Document OAuth scopes and permissions
5. Document how cloud files are attached to Aula messages/posts
6. Write cloud_storage_integration.md
7. Commit
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Documented the complete cloud storage integration architecture for Aula's Google Drive and OneDrive file operations.

Key findings:
- Provider abstraction via CloudStorageIntegrationProviderInterface with GoogleDriveViewModel and OneDriveViewModel implementations
- Two operational modes: link-attach (reference only, no download) and download-import (for gallery media)
- Google Drive uses drive.readonly scope; OneDrive uses files.Read, files.Read.all, and Sites.Read.all
- OAuth tokens cached in-memory for 2 minutes only, not persisted to storage
- File operations go directly to Google/Microsoft APIs with Bearer tokens (no Aula backend proxy)
- Aula backend permissions (ATTACH_GOOGLE_DRIVE_FILE, etc.) gate UI visibility but not API access
- Both use public client PKCE flow (empty secret)

Produced: cloud_storage_integration.md with full architecture, flow diagrams, scope documentation, security observations, and file reference table.
<!-- SECTION:FINAL_SUMMARY:END -->
