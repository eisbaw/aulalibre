---
id: TASK-0057
title: Documents and files API services
status: To Do
assignee: []
created_date: '2026-03-18 16:12'
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
- [ ] #1 Secure document service: list, get details/revisions, create/update internal, sharing management
- [ ] #2 Document operations: lock/unlock, soft delete, shareable docs
- [ ] #3 Export and PDF generation with async tracking
- [ ] #4 File service: document links, S3 upload (single and multipart), authenticated download
<!-- AC:END -->
