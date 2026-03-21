---
id: TASK-0110
title: 'aula-fuse: Replace std::env::set_var with env_logger::Builder'
status: To Do
assignee: []
created_date: '2026-03-21 06:19'
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
- [ ] #1 No std::env::set_var calls in aula-fuse
- [ ] #2 Logging level configured via env_logger::Builder API
<!-- AC:END -->
