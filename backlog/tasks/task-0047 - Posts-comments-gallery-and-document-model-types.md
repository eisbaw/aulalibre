---
id: TASK-0047
title: 'Posts, comments, gallery, and document model types'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:09'
updated_date: '2026-03-18 17:55'
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
- [x] #1 Post, CreatePostParameter, PostFilter structs
- [x] #2 Comment structs
- [x] #3 Album, Media, MediaTag structs
- [x] #4 SecureDocument, DocumentRevision, CommonFile, DocumentSharing structs
- [x] #5 AulaFileResult, FileContent, UploadFileInfo structs
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create files.rs — AulaFileContent, AulaFileResultDto, AulaFileResultProfileDto, AulaMediaFileContent, AulaLinkContent, AulaDocumentLinkContent, AulaFileAlbumDto, AulaGalleryMediaFileResultDto, AuthorizedFileFormat, FileConnectionResult, UploadFileInfo, UploadFileKeyInfo, UploadFileData, UploadLink, ShareWithGroupDto, MembershipCountResultModel, BaseResultDto, FileResultDto, LinkResultDto, CreateAttachmentsResult, CreateMediaResult, FilePartUploadInformation, FileUploadInformation, upload parameter types
2. Create gallery.rs — AlbumDto, AlbumCreatorDto, AlbumGroupDto, MediaCreatorModel, MediaListDto, MediasInAlbumDto, CreateAlbumParameters, DeleteAlbumParameters, GalleryViewFilter, GetMediaInAlbumFilter, LinkedGroupRequestModel
3. Create documents.rs — SecureDocumentDto, SecureDocumentCreatorDto, SecureDocumentAssociateGroupDto, SecureDocumentAssociateInstitutionProfileDto, SecureDocumentShareWithGroupDto, SecureDocumentShareWithInstitutionProfileDto, CommonFileDto (+ nested), DocumentRevisionDto, DocumentRevisionPageDto, ExternalSecureDocumentDetailsDto, InternalSecureDocumentDetailsDto, ImplicitSharing types, GetSecureDocumentsArguments, GetCommonFilesArguments, CreateDocumentArguments, SortingModel, UpdateSharingArguments, request types, result types, SecureDocumentExportDto
4. Create posts.rs — PostApiDto, ProfileApiDto, CreatePostApiParameter, GetPostApiParameters, GetPostApiResult, CreatePostResult, ReportApiParameter, CommentResultModel, PagedCommentList, CommentableInstitutionProfile, CommentItem, DeleteCommentRequestModel, ReportCommentApiParameters, UpdateCommentRequestModel
5. Register all modules in mod.rs
6. Run nix-shell --run just e2e
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added Rust model structs for the remaining Aula content domains: posts, comments, gallery, documents, and files.

New files:
- `files.rs` -- AulaFileContent, AulaFileResultDto, AulaMediaFileContent, ShareWithGroupDto, upload types, attachment parameter/result types (38 structs)
- `gallery.rs` -- AlbumDto, AlbumCreatorDto, MediaListDto, MediasInAlbumDto, gallery filter/parameter types (10 structs)
- `documents.rs` -- SecureDocumentDto, CommonFileDto, DocumentRevisionDto, document creation/query arguments, sharing types, export types (28 structs)
- `posts.rs` -- PostApiDto, CreatePostApiParameter, GetPostApiParameters, CommentResultModel, PagedCommentList, comment CRUD types (14 structs)

All modules registered in mod.rs. 245 tests pass (30 new), clippy clean, fmt clean.
<!-- SECTION:FINAL_SUMMARY:END -->
