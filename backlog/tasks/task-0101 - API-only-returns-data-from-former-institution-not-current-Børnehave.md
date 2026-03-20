---
id: TASK-0101
title: 'API only returns data from former institution, not current Børnehave'
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-19 21:51'
updated_date: '2026-03-19 21:52'
labels:
  - rust-cli
  - investigation
  - api
dependencies: []
references:
  - aula/aula-api/src/session.rs
  - aula/aula-api/src/services/profiles.rs
  - aula/aula-api/src/models/onboarding.rs
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The getProfilesByLogin response only returns one institution profile (X99999 Børneinstitutionen REDACTED-INST, group "REDACTED-GROUP" which is the former vuggestue). REDACTED-CHILD is currently enrolled in a Børnehave at the same institution but the API does not return data scoped to that enrollment. Albums only show photos from the vuggestue period (up to Dec 2025), messages only show vuggestue-era threads, and posts return empty.

This suggests either:
1. The getProfileContext call needs additional parameters to switch institution/group context
2. There are multiple institution profiles (one per department) and we only see one
3. The child has multiple institution profile IDs (one per department) and we need to query with the børnehave one

The HAR file from a browser session could reveal how the web frontend handles this - compare what institution profile IDs and context parameters the web app sends vs what our CLI sends.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Albums, messages, and posts from the current Børnehave enrollment are visible
- [ ] #2 Understand how institution/group context switching works in the API
- [ ] #3 CLI correctly scopes queries to the active enrollment
<!-- AC:END -->
