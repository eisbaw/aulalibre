---
id: TASK-0100
title: 'Expand search to cover posts, events, and groups (findGeneric workaround)'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 18:47'
updated_date: '2026-03-19 19:09'
labels:
  - rust-cli
  - investigation
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
search.findGeneric is broken server-side (HTTP 500). Currently we fall back to search.findProfiles which only returns profile results. Investigate whether combining multiple search endpoints (findProfiles, findGroups, findMessage) can approximate the full-text search that findGeneric was supposed to provide. Also check periodically whether findGeneric gets fixed in newer API versions.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Evaluate which additional search endpoints return useful results
- [x] #2 Implement combined search if viable, or document why not
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Probed all 6 search endpoints against live API (2026-03-19):
- search.findGeneric: HTTP 500 (server broken, "intern fejl")
- search.findProfiles: OK (returns profile results)
- search.findGroups: OK (returns group results, id field comes as string)
- search.findProfilesAndGroups: HTTP 400 (needs context params we lack)
- search.findMessage: HTTP 400 (needs context params we lack)
- search.findRecipients: HTTP 400 (needs context params we lack)

Implemented combined search fallback using findProfiles + findGroups.
Fixed SearchGroupItemResultModel.id deserialization (string vs i64).
Added --probe flag for periodic re-testing of endpoint status.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Probed all 6 search endpoints. Only findProfiles and findGroups work as standalone endpoints. findProfilesAndGroups, findMessage, findRecipients return 400 (need workflow-specific context params). Implemented combined search merging findProfiles + findGroups results. Added --probe flag. Fixed SearchGroupItemResultModel.id deserialization for string/int flexibility. Posts, events, messages remain unsearchable until findGeneric is fixed server-side.
<!-- SECTION:FINAL_SUMMARY:END -->
