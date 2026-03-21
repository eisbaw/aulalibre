---
id: TASK-0120
title: 'aula-fuse: Clean up minor code issues (dead code, missing open(), env parsing)'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:53'
labels:
  - aula-fuse
  - observation
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Minor issues from MPED review: (1) InodeEntry::ResourceItem.name duplicates children HashMap key, (2) ContentSource::Empty is never constructed, (3) No open() validation, (4) parse_environment should use clap ValueEnum with hard error on typos instead of silent fallback to production.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Remove ContentSource::Empty or use it
- [x] #2 parse_environment replaced with clap ValueEnum derive
- [x] #3 Unknown --env value is a hard error, not silent fallback to production
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed dead ContentSource::Empty variant and its match arm in fs.rs. Replaced the manual parse_environment function (which silently defaulted to production on unknown input) with a local EnvironmentArg enum deriving clap::ValueEnum. Unknown --env values now produce a hard error from clap at parse time. The EnvironmentArg wrapper avoids adding clap as a dependency to aula-api.

Files changed:
- aula-fuse/src/inode_table.rs: removed Empty variant, narrowed #[allow(dead_code)] to LazyDownload only
- aula-fuse/src/fs.rs: removed Empty match arm
- aula-fuse/src/main.rs: replaced parse_environment with EnvironmentArg ValueEnum + From impl

Tests: 40 passed, clippy clean with -D warnings.
<!-- SECTION:FINAL_SUMMARY:END -->
