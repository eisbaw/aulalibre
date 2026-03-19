---
id: TASK-0099
title: Investigate search endpoint HTTP 500 internal error
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 17:10'
updated_date: '2026-03-19 18:53'
labels:
  - rust-cli
  - investigation
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
search "menu" returns HTTP 500 "intern fejl" (internal server error). Could be request format issue, missing required fields, or wrong parameter encoding. Need to compare with the web frontend's search request format from the HAR capture or decompiled app.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Root cause of search 500 error identified
- [x] #2 search command returns results or clear error if server-side issue
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Investigation complete. Root cause: search.findGeneric is broken server-side in the Aula production backend. Returns HTTP 500 "intern fejl" regardless of parameters (tested: text, pageLimit, pageNumber, docTypeCount, docType, instProfileIds[]). This is not a client-side bug.

Fixes applied:
1. aula-api/src/client.rs: Added HTTP 5xx early error handling. Previously, a 500 response with valid JSON envelope {"status":{"code":0,...},"data":"intern fejl"} was treated as success because backend_error_code was 0.
2. aula-api/src/services/search.rs: Fixed search_for_profiles return type from Vec<SearchResultProfileItemGlobalSearch> to SearchResponse (actual API returns object, not array as declared in decompiled .NET).
3. aula-cli/src/commands/search.rs: Rewrote with fallback pattern - try findGeneric first, fall back to findProfiles when it fails.

Limitation: findProfiles only returns profile results, not posts/events/groups/messages that findGeneric would provide. The --counts flag produces no useful output since findProfiles does not populate doc_type_count.

All tests pass (669+), clippy clean. Live testing confirms search works for profile queries.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Root cause: search.findGeneric returns HTTP 500 server-side (not fixable client-side). Fixed by: (1) adding HTTP 5xx rejection in client.rs handle_response(), (2) fixing search_for_profiles return type to SearchResponse, (3) implementing fallback pattern in CLI - tries findGeneric first, falls back to findProfiles. Created TASK-0100 for expanding search via other endpoints.
<!-- SECTION:FINAL_SUMMARY:END -->
