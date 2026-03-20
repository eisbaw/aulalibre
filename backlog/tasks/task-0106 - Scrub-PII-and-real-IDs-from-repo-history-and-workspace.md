---
id: TASK-0106
title: Scrub PII and real IDs from repo history and workspace
status: Done
assignee:
  - '@claude'
created_date: '2026-03-20 21:43'
updated_date: '2026-03-20 22:01'
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
- [x] #4 Run git filter-repo to rewrite history, producing a new branch main-scrubbed with all PII removed from all commits
- [x] #5 Verify main-scrubbed is clean: re-run PII scan across workspace and full git history
- [x] #6 After verification, delete old main branch, rename main-scrubbed to main, run git gc --aggressive --prune=now
- [x] #7 Confirm no .har, .pcap, or secrets/ content is in git history
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

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Scrubbed all PII and real Aula IDs from the repository workspace and full git history.

Changes:
- Replaced real profile IDs, institution profile IDs, and institution code with synthetic values in onboarding.rs and service_integration_tests.rs test fixtures
- Real IDs preserved in gitignored secrets/test_fixtures.json for local use
- Scrubbed backlog tasks 0091, 0097, 0101 to remove institution names, codes, and profile IDs
- Ran git-filter-repo with text replacements across all commits
- Deleted stale v0.1 tag pointing to unscrubbed history, re-created on scrubbed commit
- Ran git gc --aggressive --prune=now to purge old objects

Verification:
- Full PII scan (12 terms) across workspace and git history: clean
- No .har, .pcap, or secrets/ content in git history
- All 456+ tests pass, clippy and fmt clean
- Only PII remaining is in gitignored secrets/ directory (expected)
<!-- SECTION:FINAL_SUMMARY:END -->
