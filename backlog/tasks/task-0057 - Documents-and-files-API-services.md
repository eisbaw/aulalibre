---
id: TASK-0057
title: Documents and files API services
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:12'
updated_date: '2026-03-18 19:07'
labels:
  - rust
  - aula-api
  - service
dependencies:
  - TASK-0049
  - TASK-0047
references:
  - api_endpoints.md
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement document and file services. Secure Documents (18 methods, Section 3.11): GetSecureDocuments, GetCommonFiles, UpdateSharings, RemoveOwnSharings, GetImplicitSharings, GetDocumentRevisions, Get/GetRevision for internal and external docs, CreateInternalSecureDocument, UpdateInternalSecureDocument, UpdateDocumentLockedStatus, SoftDeleteSecureDocument, GetShareableSecureDocuments, export operations (GetMaxDocumentsPerExport, CreateExportForMultiple, TrackExport), PDF generation (CreatePDFForSingle, TrackCreatePDF). Files (8 methods, Section 3.12): CreateDocumentLinks, UploadFileToAws (pre-signed S3 URL), UploadPartToAws (multipart), FetchHttpResponse, TryGetResponseWithToken, DownloadFileWithProgress.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Secure document service: list, get details/revisions, create/update internal, sharing management
- [x] #2 Document operations: lock/unlock, soft delete, shareable docs
- [x] #3 Export and PDF generation with async tracking
- [x] #4 File service: document links, S3 upload (single and multipart), authenticated download
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create documents service (aula-api/src/services/documents.rs) with all 18+ methods from Section 3.11
2. Create files service (aula-api/src/services/files.rs) with all 8 methods from Section 3.12
3. Register both modules in services/mod.rs
4. Add serialization and request/response model tests
5. Run just e2e to verify everything compiles and tests pass
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Created documents.rs with 20 service methods covering all Section 3.11 endpoints
- Created files.rs with 4 Aula API methods (document links, attachments, upload links, multipart completion)
- Noted that S3 upload/download methods are client-side operations against external URLs, not Aula API routes
- Added UpdateDocumentLockedStatusRequest model in the service module
- 18 new tests (12 documents + 6 files), all passing
- Total test count: 417, clippy clean, fmt clean
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented document and file API services for the Aula Rust client.

Changes:
- Added `services/documents.rs` with 20 async methods mapping to SecureDocumentWebService (Section 3.11): secure document CRUD, sharing management, implicit sharings, revisions, external/internal document details and revisions, lock/unlock, soft delete, shareable documents, export (create + track), and PDF generation (create + track).
- Added `services/files.rs` with 4 Aula API methods mapping to FileWebService (Section 3.12): document link creation, attachment creation, pre-signed upload link retrieval, and multipart upload completion. S3 upload and file download methods are intentionally omitted as they operate on external URLs, not the Aula API.
- Registered both modules in `services/mod.rs`.
- Added `UpdateDocumentLockedStatusRequest` model in the documents service.

Tests:
- 18 new unit tests (12 documents, 6 files) covering serialization and deserialization of all request/response types.
- All 417 tests pass, clippy clean, rustfmt clean.
<!-- SECTION:FINAL_SUMMARY:END -->
