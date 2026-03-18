---
id: TASK-0056
title: 'Posts, comments, and gallery API services'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:11'
updated_date: '2026-03-18 19:00'
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
Implement three smaller API services. Posts (8 methods, Section 3.8): GetPosts, GetPostById, CreatePost, EditPost, DeletePost, ReportPost, BookmarkPost, UnbookmarkPost. Comments (5 methods, Section 3.9): AddComment, UpdateComment, GetComments, ReportComment, DeleteComment. Gallery (12 methods, Section 3.10): GetAlbums, GetMediasInAlbum, GetMediaById, CreateAlbum, UpdateAlbum, DeleteAlbum, DeleteMedia, AddTag, RemoveTag, ReportMedia. These are grouped together because they are moderately sized services that share content/social patterns.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Post service: CRUD, report, bookmark/unbookmark
- [x] #2 Comment service: CRUD, report
- [x] #3 Gallery service: album CRUD, media operations, tagging, reporting
- [x] #4 All methods return properly typed Results
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create posts.rs service: 8 methods (get_posts, get_post_by_id, create_post, edit_post, delete_post, report_post, bookmark_post, unbookmark_post)
2. Create comments.rs service: 5 methods (add_comment, update_comment, get_comments, report_comment, delete_comment)
3. Create gallery.rs service: 12 methods (get_albums, get_albums_cached, get_medias_in_album, get_medias_in_album_cached, get_media_by_id, create_album, update_album, delete_album, delete_media, add_tag, remove_tag, report_media)
4. Register all three modules in mod.rs
5. Run e2e tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented all three services following existing patterns (profiles.rs, messaging.rs).
- posts.rs: 8 methods with query-param building for get_posts
- comments.rs: 5 methods with AddCommentRequestModel and GetCommentsRequestModel as service-local types
- gallery.rs: 12 methods including cached variants that delegate to non-cached
- All methods return typed Results
- 399 tests pass, clippy clean, fmt clean
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented posts, comments, and gallery API services for the Aula API client.

Changes:
- Added `posts.rs` (8 methods): get_posts with query param building, get_post_by_id, create_post, edit_post, delete_post, report_post, bookmark_post, unbookmark_post
- Added `comments.rs` (5 methods): add_comment, update_comment, get_comments with query params, report_comment, delete_comment. Includes service-local AddCommentRequestModel and GetCommentsRequestModel types matching the decompiled Services.Web namespace.
- Added `gallery.rs` (12 methods): get_albums/cached, get_medias_in_album/cached, get_media_by_id, create/update/delete_album, delete_media, add_tag, remove_tag, report_media. Cached variants delegate to non-cached since caching is client-side.
- Registered all three modules in services/mod.rs

All methods return properly typed crate::Result<T>. Reuses existing model types from models/posts.rs, models/gallery.rs, and models/files.rs.

Tests: 399 pass, clippy clean, rustfmt clean.
<!-- SECTION:FINAL_SUMMARY:END -->
