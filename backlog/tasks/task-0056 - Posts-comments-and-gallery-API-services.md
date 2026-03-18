---
id: TASK-0056
title: 'Posts, comments, and gallery API services'
status: To Do
assignee: []
created_date: '2026-03-18 16:11'
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
- [ ] #1 Post service: CRUD, report, bookmark/unbookmark
- [ ] #2 Comment service: CRUD, report
- [ ] #3 Gallery service: album CRUD, media operations, tagging, reporting
- [ ] #4 All methods return properly typed Results
<!-- AC:END -->
