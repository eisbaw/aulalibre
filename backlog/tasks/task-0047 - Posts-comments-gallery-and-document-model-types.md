---
id: TASK-0047
title: 'Posts, comments, gallery, and document model types'
status: To Do
assignee: []
created_date: '2026-03-18 16:09'
labels:
  - rust
  - aula-api
  - models
dependencies:
  - TASK-0042
references:
  - data_models.md
  - domain_concepts.md
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Rust structs for remaining content domains. Posts: PostApiDto, CreatePostApiParameter, PostFilter. Comments: Comment types. Gallery: AlbumDto, MediaListDto, MediaTag, ConversionStatus. Documents: SecureDocumentDto, DocumentRevisionDto, CommonFileDto, DocumentSharing, JournalingStatus. Files: AulaFileResultDto, AulaFileContent, UploadFileInfo, FileConnectionResult, AuthorizedFileFormat. See data_models.md for Models.Posts.Api, Models.Gallery, Models.Document, and Models.Common.Api.Files namespaces.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Post, CreatePostParameter, PostFilter structs
- [ ] #2 Comment structs
- [ ] #3 Album, Media, MediaTag structs
- [ ] #4 SecureDocument, DocumentRevision, CommonFile, DocumentSharing structs
- [ ] #5 AulaFileResult, FileContent, UploadFileInfo structs
<!-- AC:END -->
