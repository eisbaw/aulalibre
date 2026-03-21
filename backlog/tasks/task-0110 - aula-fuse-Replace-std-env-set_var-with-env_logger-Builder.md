---
id: TASK-0110
title: 'aula-fuse: Replace std::env::set_var with env_logger::Builder'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-21 06:19'
updated_date: '2026-03-21 06:26'
labels:
  - aula-fuse
  - critical
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
main.rs uses std::env::set_var to set RUST_LOG before env_logger::init(). This is unsound in multi-threaded contexts and deprecated in Rust 2024 edition. Use env_logger::Builder::new().filter_level() directly instead.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 No std::env::set_var calls in aula-fuse
- [x] #2 Logging level configured via env_logger::Builder API
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Replaced unsafe std::env::set_var("RUST_LOG", ...) + env_logger::init() with env_logger::Builder::new().filter_level(level).init() in aula-fuse main.rs.

This eliminates the unsound set_var call that is deprecated in Rust 2024 edition and unsafe in multi-threaded contexts. The log level is now configured directly via the Builder API based on the --verbose flag.

Tests: 32/32 pass, clippy clean (zero warnings).
<!-- SECTION:FINAL_SUMMARY:END -->
