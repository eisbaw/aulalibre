---
id: TASK-0106
title: Scrub PII and real IDs from repo history and workspace
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-20 21:43'
updated_date: '2026-03-20 21:46'
labels:
  - security
  - pii
  - git
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Real profile/institution IDs and PII are present in tracked files (test fixtures and backlog task descriptions). These must be removed from both the current workspace and the full git history before the repo can be made public.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Scrub backlog task 0091 description: remove real institution name, code, municipality, and profile ID
- [x] #2 Scrub backlog task 0101 description: remove real institution name, code, group name, and child references
- [x] #3 Scrub backlog task 0097 AC: remove real profile ID
- [ ] #4 Run git filter-repo to rewrite history, producing a new branch main-scrubbed with all PII removed from all commits
- [ ] #5 Verify main-scrubbed is clean: re-run PII scan across workspace and full git history
- [ ] #6 After verification, delete old main branch, rename main-scrubbed to main, run git gc --aggressive --prune=now
- [ ] #7 Confirm no .har, .pcap, or secrets/ content is in git history
- [x] #8 Move real IDs from onboarding.rs test fixtures to secrets/test_fixtures.json (done, tests use synthetic IDs now)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Move real IDs from onboarding.rs test fixtures to secrets/test_fixtures.json, load at test time
2. Scrub backlog tasks 0091, 0101, 0097 via CLI
3. Commit all workspace changes
4. Install git-filter-repo (check nix availability)
5. Run git filter-repo to produce main-scrubbed branch
6. Verify main-scrubbed is clean (full PII scan)
7. Delete old main, rename, git gc --aggressive --prune=now
8. Confirm no .har/.pcap/secrets in history
<!-- SECTION:PLAN:END -->
